//! Human-readable presentation of driver errors.
//!
//! This module is a presentation layer only: it never changes which programs
//! are accepted. Driver diagnostics keep their structured form in
//! [`DriverError`]; rendering resolves their spans and AST nodes back to
//! source files, lines, and columns. Diagnostic codes are the snake_case
//! names of the diagnostic kinds, matching the names used in `docs/SPEC.md`.

use std::{
    collections::HashSet,
    fmt::{Debug, Write as _},
    path::{Path, PathBuf},
};

use crate::{
    ast::AstNodeId,
    driver::{DriverError, TestMetadata},
    module::{PackageGraphDiagnosticKind, VirtualPackageGraph, VirtualSource},
    parser::{ParseOutput, parse_source},
    source::{ByteSpan, SourceFileId},
    type_check::{EntryPointDiagnosticLocation, TypeRuleDiagnostic},
};

/// The source file and one-based line that produced a line of compiled text.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LineOrigin {
    pub path: PathBuf,
    pub line: usize,
}

/// Compiled source text together with the parse output used to resolve AST
/// nodes and the origin of each of its lines.
pub struct SourceView {
    file: SourceFileId,
    text: String,
    origins: Vec<LineOrigin>,
    parsed: ParseOutput,
}

impl SourceView {
    pub fn new(file: SourceFileId, text: impl Into<String>, origins: Vec<LineOrigin>) -> Self {
        let text = text.into();
        let parsed = parse_source(file, &text);
        Self {
            file,
            text,
            origins,
            parsed,
        }
    }

    /// A view of a single file whose lines are reported as-is.
    pub fn single(file: SourceFileId, path: impl Into<PathBuf>, text: impl Into<String>) -> Self {
        let text = text.into();
        let path = path.into();
        let origins = identity_origins(&path, &text);
        Self::new(file, text, origins)
    }

    fn span_of(&self, node: AstNodeId) -> Option<ByteSpan> {
        self.parsed.arena.node(node).map(|node| node.span)
    }
}

fn identity_origins(path: &Path, text: &str) -> Vec<LineOrigin> {
    (1..=text.split('\n').count())
        .map(|line| LineOrigin {
            path: path.to_path_buf(),
            line,
        })
        .collect()
}

#[derive(Clone, Debug)]
enum Location {
    Span(ByteSpan),
    Node(AstNodeId),
    Path(PathBuf),
}

#[derive(Clone, Debug)]
struct Report {
    code: Option<String>,
    message: String,
    location: Option<Location>,
    notes: Vec<(String, Location)>,
}

impl Report {
    fn new(code: impl Debug, message: impl Into<String>, location: Option<Location>) -> Self {
        Self {
            code: Some(snake_case(&format!("{code:?}"))),
            message: message.into(),
            location,
            notes: Vec::new(),
        }
    }

    fn kind(kind: impl Debug, location: Option<Location>) -> Self {
        let message = humanize(&format!("{kind:?}"));
        Self::new(kind, message, location)
    }

    fn plain(message: impl Into<String>) -> Self {
        Self {
            code: None,
            message: message.into(),
            location: None,
            notes: Vec::new(),
        }
    }
}

/// Renders an error produced while compiling a manifest project.
///
/// The manifest's sources are reloaded so that spans and AST nodes can be
/// mapped back to file, line, and column. If the project can no longer be
/// loaded, diagnostics are still rendered without source excerpts.
pub fn render_manifest_error(manifest_path: impl AsRef<Path>, error: &DriverError) -> String {
    let context = ManifestContext::load(manifest_path.as_ref());
    let reports = reports_for(error, &context);
    let mut output = render_reports(&reports, &context);
    if let Some(name) = &context.name {
        let count = reports.len();
        let plural = if count == 1 { "" } else { "s" };
        let _ = writeln!(
            output,
            "error: could not compile `{name}` due to {count} previous error{plural}"
        );
    }
    output
}

/// Renders an error produced while compiling one discovered test.
pub fn render_test_error(test: &TestMetadata, file: SourceFileId, error: &DriverError) -> String {
    let context = ManifestContext {
        name: None,
        views: vec![SourceView::new(
            file,
            test.source(),
            test.line_origins().to_vec(),
        )],
        files: Vec::new(),
    };
    render_reports(&reports_for(error, &context), &context)
}

struct ManifestContext {
    name: Option<String>,
    /// Views used to resolve spans, searched in order.
    views: Vec<SourceView>,
    /// Individual project files, used for package-graph parse diagnostics.
    files: Vec<(SourceFileId, PathBuf, String)>,
}

impl ManifestContext {
    fn load(manifest_path: &Path) -> Self {
        let mut context = Self {
            name: None,
            views: Vec::new(),
            files: Vec::new(),
        };
        let Ok((manifest, root)) = crate::manifest::ProjectManifest::load(manifest_path) else {
            return context;
        };
        context.name = Some(manifest.name().to_owned());
        let Ok(sources) = manifest.load_sources(&root) else {
            return context;
        };
        let virtual_sources = sources
            .iter()
            .map(|source| VirtualSource::new(source.path(), source.source()))
            .collect::<Vec<_>>();
        // Source ids are assigned by the package graph; a library build
        // assigns the same ids without requiring the entry graph to be valid.
        if let Ok(library) = VirtualPackageGraph::build_library(virtual_sources.clone()) {
            for file in library.files() {
                if let Some(source) = sources.iter().find(|source| source.path() == file.path) {
                    context
                        .files
                        .push((file.id, file.path.clone(), source.source().to_owned()));
                }
            }
        } else {
            for (index, source) in sources.iter().enumerate() {
                context.files.push((
                    SourceFileId::from_raw(index),
                    source.path().to_path_buf(),
                    source.source().to_owned(),
                ));
            }
        }
        if let Some(entry) = manifest.entrypoint() {
            let dependencies = crate::dependency::GitDependencyResolver::from_environment()
                .and_then(|resolver| resolver.load_project_dependencies(manifest_path))
                .unwrap_or_default();
            if let Ok(graph) = VirtualPackageGraph::build_with_dependencies(
                entry.to_path_buf(),
                sources,
                dependencies,
            ) && let Some(file) = graph.files().iter().find(|file| file.path == entry)
            {
                context.views.push(SourceView::new(
                    file.id,
                    graph.bootstrap_source(),
                    graph.bootstrap_line_origins(),
                ));
            }
        }
        for (id, path, text) in &context.files {
            context
                .views
                .push(SourceView::single(*id, path.clone(), text.clone()));
        }
        context
    }

    fn view_for_span(&self, span: ByteSpan) -> Option<&SourceView> {
        self.views
            .iter()
            .find(|view| view.file == span.file() && span.end() <= view.text.len())
    }

    fn span_of(&self, node: AstNodeId) -> Option<(ByteSpan, &SourceView)> {
        self.views.iter().find_map(|view| {
            let span = view.span_of(node)?;
            Some((span, view))
        })
    }
}

fn reports_for(error: &DriverError, context: &ManifestContext) -> Vec<Report> {
    let span = |span: ByteSpan| Some(Location::Span(span));
    let node = |node: AstNodeId| Some(Location::Node(node));
    let mut reports = match error {
        DriverError::Io { operation, path } => vec![Report::plain(format!(
            "failed to {operation} `{}`",
            path.display()
        ))],
        DriverError::LexDiagnostics(diagnostics) => diagnostics
            .iter()
            .map(|diagnostic| Report::kind(diagnostic.kind, span(diagnostic.span)))
            .collect(),
        DriverError::ParseDiagnostics(diagnostics) => diagnostics
            .iter()
            .map(|diagnostic| Report::kind(diagnostic.kind, span(diagnostic.span)))
            .collect(),
        DriverError::OutOfScopeNames(names) => names
            .iter()
            .map(|name| {
                Report::new(
                    "OutOfScopeName",
                    format!("`{}` is not in scope here", name.name),
                    span(name.span),
                )
            })
            .collect(),
        DriverError::UnknownNames(names) => names
            .iter()
            .map(|name| {
                Report::new(
                    "UnknownName",
                    format!("`{}` is not declared in this program", name.name),
                    span(name.span),
                )
            })
            .collect(),
        DriverError::EntryPointDiagnostics(diagnostics) => diagnostics
            .iter()
            .map(|diagnostic| match diagnostic.location() {
                EntryPointDiagnosticLocation::Source(location) => {
                    Report::kind(diagnostic.kind(), span(*location))
                }
                EntryPointDiagnosticLocation::EntryPackageInput(package) => {
                    let mut report = Report::kind(diagnostic.kind(), None);
                    let package = package.as_str();
                    if !package.is_empty() {
                        report.message.push_str(&format!(" in package `{package}`"));
                    }
                    report
                }
            })
            .collect(),
        DriverError::ReturnPathDiagnostics(diagnostics) => diagnostics
            .iter()
            .map(|diagnostic| Report::kind(diagnostic.kind(), node(diagnostic.node())))
            .collect(),
        DriverError::ReturnTypeDiagnostics(diagnostics) => diagnostics
            .iter()
            .map(|diagnostic| Report::kind(diagnostic.kind(), span(diagnostic.span())))
            .collect(),
        DriverError::UnsupportedExecutableForms(diagnostics) => diagnostics
            .iter()
            .map(|diagnostic| {
                Report::new(
                    "UnsupportedExecutableForm",
                    "this form is not supported in executables yet",
                    span(diagnostic.span()),
                )
            })
            .collect(),
        DriverError::TypeDiagnostics(diagnostics) => diagnostics
            .iter()
            .map(|diagnostic| {
                let message = match diagnostic.rule() {
                    TypeRuleDiagnostic::LambdaCaptureUnsupported => {
                        // Spec: ADR-0088 and ADR-0089 define these capture rules.
                        "closure capture not supported: captured values must be copied or \
                         transferred, and values captured by a spawned task must be Send"
                            .to_owned()
                    }
                    rule => humanize(&format!("{rule:?}")),
                };
                Report::new(diagnostic.kind(), message, node(diagnostic.node()))
            })
            .collect(),
        DriverError::TestDiagnostics(diagnostics) => diagnostics
            .iter()
            .map(|diagnostic| Report::kind(diagnostic.kind(), span(diagnostic.span())))
            .collect(),
        DriverError::IntrinsicDiagnostics(diagnostics) => diagnostics
            .iter()
            .map(|diagnostic| Report::kind(diagnostic.kind(), span(diagnostic.span())))
            .collect(),
        DriverError::ConstructorDiagnostics(diagnostics) => diagnostics
            .iter()
            .map(|diagnostic| Report::kind(diagnostic.kind(), span(diagnostic.span())))
            .collect(),
        DriverError::DispatchDiagnostics(diagnostics) => diagnostics
            .iter()
            .map(|diagnostic| Report::kind(diagnostic.kind(), span(diagnostic.span())))
            .collect(),
        DriverError::DirectCallDiagnostics(diagnostics) => diagnostics
            .iter()
            .map(|diagnostic| Report::kind(diagnostic.kind(), span(diagnostic.span())))
            .collect(),
        DriverError::OwnershipDiagnostics(diagnostics) => diagnostics
            .iter()
            .map(|diagnostic| {
                let kind = diagnostic.kind();
                let message = match kind {
                    crate::ownership::OwnershipDiagnosticKind::UseAfterMove => {
                        "value used after it was moved".to_owned()
                    }
                };
                let mut report = Report::new(kind, message, node(diagnostic.node()));
                report.notes.push((
                    "value moved here".to_owned(),
                    Location::Node(diagnostic.move_origin()),
                ));
                report
            })
            .collect(),
        DriverError::Hir(error) => vec![internal_report("HIR lowering", error)],
        DriverError::Mir(error) => vec![internal_report("MIR lowering", error)],
        DriverError::Backend(error) => vec![internal_report("code generation", error)],
        DriverError::HostOnlyTarget(target) => vec![Report::new(
            "HostOnlyTarget",
            format!("target `{target}` is not supported; only the host target can be built"),
            None,
        )],
        DriverError::Link(error) => vec![Report::new(
            "LinkFailed",
            format!("linking failed: {}", humanize(&format!("{error:?}"))),
            None,
        )],
        DriverError::PackageGraph(diagnostics) => diagnostics
            .iter()
            .flat_map(|diagnostic| package_graph_reports(diagnostic, context))
            .collect(),
        DriverError::Manifest(diagnostic) => {
            vec![Report::new(diagnostic.kind(), diagnostic.detail(), None)]
        }
        DriverError::Dependency(diagnostic) => {
            vec![Report::new(diagnostic.kind(), diagnostic.detail(), None)]
        }
    };
    dedupe(&mut reports, context);
    reports
}

fn internal_report(phase: &str, error: &impl Debug) -> Report {
    let mut report = Report::plain(format!(
        "{phase} failed: {}",
        humanize(&format!("{error:?}"))
    ));
    report.code = Some(snake_case(&format!("{error:?}")));
    report
}

fn package_graph_reports(
    diagnostic: &crate::module::PackageGraphDiagnostic,
    context: &ManifestContext,
) -> Vec<Report> {
    if diagnostic.kind == PackageGraphDiagnosticKind::SourceParseDiagnostics
        && let Some((id, _, text)) = context
            .files
            .iter()
            .find(|(_, path, _)| path == &diagnostic.path)
    {
        let parsed = parse_source(*id, text);
        let reports = parsed
            .lex_diagnostics
            .iter()
            .map(|diagnostic| Report::kind(diagnostic.kind, Some(Location::Span(diagnostic.span))))
            .chain(parsed.diagnostics.iter().map(|diagnostic| {
                Report::kind(diagnostic.kind, Some(Location::Span(diagnostic.span)))
            }))
            .collect::<Vec<_>>();
        if !reports.is_empty() {
            return reports;
        }
    }
    let location = match diagnostic.span {
        Some(span) => Location::Span(span),
        None => Location::Path(diagnostic.path.clone()),
    };
    vec![Report::new(
        diagnostic.kind,
        diagnostic.detail.clone(),
        Some(location),
    )]
}

fn dedupe(reports: &mut Vec<Report>, context: &ManifestContext) {
    let mut seen = HashSet::new();
    reports.retain(|report| {
        let location = report
            .location
            .as_ref()
            .and_then(|location| resolve(location, context))
            .map(|resolved| (resolved.path, resolved.line, resolved.column));
        seen.insert((report.code.clone(), report.message.clone(), location))
    });
}

struct Resolved {
    path: PathBuf,
    line: usize,
    column: usize,
    excerpt: Option<(String, usize)>,
}

fn resolve(location: &Location, context: &ManifestContext) -> Option<Resolved> {
    let (span, view) = match location {
        Location::Path(path) => {
            return Some(Resolved {
                path: path.clone(),
                line: 0,
                column: 0,
                excerpt: None,
            });
        }
        Location::Span(span) => (*span, context.view_for_span(*span)?),
        Location::Node(node) => context.span_of(*node)?,
    };
    let text = &view.text;
    let start = span.start().min(text.len());
    if !text.is_char_boundary(start) {
        return None;
    }
    let line_index = text[..start].matches('\n').count();
    let line_start = text[..start].rfind('\n').map_or(0, |index| index + 1);
    let line_end = text[start..]
        .find('\n')
        .map_or(text.len(), |index| start + index);
    let line_text = &text[line_start..line_end];
    let column = text[line_start..start].chars().count() + 1;
    let end = span.end().clamp(start, line_end);
    let width = text
        .get(start..end)
        .map_or(1, |covered| covered.chars().count().max(1));
    let origin = view.origins.get(line_index)?;
    Some(Resolved {
        path: origin.path.clone(),
        line: origin.line,
        column,
        excerpt: Some((line_text.to_owned(), width)),
    })
}

fn render_reports(reports: &[Report], context: &ManifestContext) -> String {
    let mut output = String::new();
    for report in reports {
        match &report.code {
            Some(code) => {
                let _ = writeln!(output, "error[{code}]: {}", report.message);
            }
            None => {
                let _ = writeln!(output, "error: {}", report.message);
            }
        }
        if let Some(location) = &report.location {
            render_location(&mut output, location, context);
        }
        for (message, location) in &report.notes {
            let _ = writeln!(output, "note: {message}");
            render_location(&mut output, location, context);
        }
        output.push('\n');
    }
    output
}

fn render_location(output: &mut String, location: &Location, context: &ManifestContext) {
    let Some(resolved) = resolve(location, context) else {
        return;
    };
    let path = resolved.path.display();
    let Some((line_text, width)) = resolved.excerpt else {
        let _ = writeln!(output, "  --> {path}");
        return;
    };
    let line = resolved.line.to_string();
    let gutter = " ".repeat(line.len());
    let _ = writeln!(output, "{gutter}--> {path}:{line}:{}", resolved.column);
    let _ = writeln!(output, "{gutter} |");
    let _ = writeln!(output, "{line} | {line_text}");
    let padding = line_text
        .chars()
        .take(resolved.column - 1)
        .map(|character| if character == '\t' { '\t' } else { ' ' })
        .collect::<String>();
    let _ = writeln!(output, "{gutter} | {padding}{}", "^".repeat(width));
}

/// Converts a `Debug` name such as `UseAfterMove` or `Ambiguous(Foo)` to
/// `use_after_move` or `ambiguous_foo`.
fn snake_case(name: &str) -> String {
    let mut output = String::new();
    let mut previous_lower = false;
    for character in name.chars() {
        if character.is_ascii_uppercase() {
            if previous_lower {
                output.push('_');
            }
            output.push(character.to_ascii_lowercase());
            previous_lower = false;
        } else if character.is_ascii_alphanumeric() {
            output.push(character);
            previous_lower = true;
        } else if !output.ends_with('_') && !output.is_empty() {
            output.push('_');
            previous_lower = false;
        }
    }
    output.trim_end_matches('_').to_owned()
}

fn humanize(name: &str) -> String {
    snake_case(name).replace('_', " ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snake_case_handles_nested_debug_names() {
        assert_eq!(snake_case("UseAfterMove"), "use_after_move");
        assert_eq!(
            snake_case("Ambiguous(OverloadSet)"),
            "ambiguous_overload_set"
        );
        assert_eq!(snake_case("LinkerFailed(Some(1))"), "linker_failed_some_1");
    }
}
