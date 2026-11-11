//! Entry points, return paths and types, and unsupported executable forms.

use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EntryPointDiagnosticKind {
    MissingEntryPoint,
    DuplicateEntryPoint,
    InvalidEntryPointSignature,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EntryPointDiagnosticLocation {
    Source(ByteSpan),
    EntryPackageInput(PackageNamespace),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntryPointDiagnostic {
    pub(crate) kind: EntryPointDiagnosticKind,
    pub(crate) location: EntryPointDiagnosticLocation,
}

impl EntryPointDiagnostic {
    pub(crate) fn source(kind: EntryPointDiagnosticKind, span: ByteSpan) -> Self {
        Self {
            kind,
            location: EntryPointDiagnosticLocation::Source(span),
        }
    }

    pub(crate) fn entry_package_input(package: &PackageNamespace) -> Self {
        Self {
            kind: EntryPointDiagnosticKind::MissingEntryPoint,
            location: EntryPointDiagnosticLocation::EntryPackageInput(package.clone()),
        }
    }

    pub fn kind(&self) -> EntryPointDiagnosticKind {
        self.kind
    }

    pub fn source_span(&self) -> Option<ByteSpan> {
        match &self.location {
            EntryPointDiagnosticLocation::Source(span) => Some(*span),
            EntryPointDiagnosticLocation::EntryPackageInput(_) => None,
        }
    }

    pub fn location(&self) -> &EntryPointDiagnosticLocation {
        &self.location
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EntryPoint {
    pub(crate) declaration: AstNodeId,
    pub(crate) source_span: ByteSpan,
}

impl EntryPoint {
    pub fn declaration(self) -> AstNodeId {
        self.declaration
    }

    pub fn source_span(self) -> ByteSpan {
        self.source_span
    }
}

pub struct EntryPointFile<'a> {
    pub(crate) package: &'a PackageNamespace,
    pub(crate) parsed: &'a ParseOutput,
}

impl<'a> EntryPointFile<'a> {
    pub fn new(package: &'a PackageNamespace, parsed: &'a ParseOutput) -> Self {
        Self { package, parsed }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EntryPointReport {
    pub(crate) entry_point: Option<EntryPoint>,
    pub(crate) diagnostics: Vec<EntryPointDiagnostic>,
}

impl EntryPointReport {
    pub fn entry_point(&self) -> Option<EntryPoint> {
        self.entry_point
    }

    pub fn diagnostics(&self) -> &[EntryPointDiagnostic] {
        &self.diagnostics
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReturnPathDiagnosticKind {
    MissingReturn,
    UnreachableReturn,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReturnPathDiagnostic {
    pub(crate) kind: ReturnPathDiagnosticKind,
    pub(crate) node: AstNodeId,
}

impl ReturnPathDiagnostic {
    pub(crate) fn new(kind: ReturnPathDiagnosticKind, node: AstNodeId) -> Self {
        Self { kind, node }
    }

    pub fn kind(self) -> ReturnPathDiagnosticKind {
        self.kind
    }

    pub fn node(self) -> AstNodeId {
        self.node
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ReturnPathReport {
    pub(crate) diagnostics: Vec<ReturnPathDiagnostic>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReturnTypeDiagnosticKind {
    ReturnTypeMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReturnTypeDiagnostic {
    pub(crate) kind: ReturnTypeDiagnosticKind,
    pub(crate) span: ByteSpan,
}

impl ReturnTypeDiagnostic {
    pub(crate) fn new(kind: ReturnTypeDiagnosticKind, span: ByteSpan) -> Self {
        Self { kind, span }
    }

    pub fn kind(self) -> ReturnTypeDiagnosticKind {
        self.kind
    }

    pub fn span(self) -> ByteSpan {
        self.span
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ReturnTypeReport {
    pub(crate) diagnostics: Vec<ReturnTypeDiagnostic>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnsupportedExecutableFormDiagnostic {
    pub(crate) span: ByteSpan,
}

impl UnsupportedExecutableFormDiagnostic {
    pub(crate) fn new(span: ByteSpan) -> Self {
        Self { span }
    }

    pub fn span(self) -> ByteSpan {
        self.span
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct UnsupportedExecutableFormReport {
    pub(crate) diagnostics: Vec<UnsupportedExecutableFormDiagnostic>,
}

impl ReturnPathReport {
    pub fn diagnostics(&self) -> &[ReturnPathDiagnostic] {
        &self.diagnostics
    }
}

impl ReturnTypeReport {
    pub fn diagnostics(&self) -> &[ReturnTypeDiagnostic] {
        &self.diagnostics
    }
}

impl UnsupportedExecutableFormReport {
    pub fn diagnostics(&self) -> &[UnsupportedExecutableFormDiagnostic] {
        &self.diagnostics
    }
}

pub fn check_entry_point(
    entry_package: &PackageNamespace,
    files: &[EntryPointFile<'_>],
) -> EntryPointReport {
    let candidates: Vec<_> = files
        .iter()
        .filter(|file| file.package == entry_package)
        .flat_map(|file| {
            file.parsed
                .function_declarations
                .iter()
                .filter(move |function| {
                    function.top_level
                        && file.parsed.declaration_names.iter().any(|name| {
                            name.declaration == function.declaration && name.name == "main"
                        })
                })
                .map(move |function| (file.parsed, function))
        })
        .collect();

    if candidates.is_empty() {
        return EntryPointReport {
            entry_point: None,
            diagnostics: vec![EntryPointDiagnostic::entry_package_input(entry_package)],
        };
    }

    if candidates.len() > 1 {
        return EntryPointReport {
            entry_point: None,
            diagnostics: candidates
                .iter()
                .map(|(parsed, function)| {
                    EntryPointDiagnostic::source(
                        EntryPointDiagnosticKind::DuplicateEntryPoint,
                        parsed
                            .arena
                            .node(function.declaration)
                            .expect("parsed function declaration is in its arena")
                            .span,
                    )
                })
                .collect(),
        };
    }

    let (parsed, candidate) = candidates[0];
    let has_int_return = candidate.return_annotation.is_some_and(|annotation| {
        parsed
            .type_name_references
            .iter()
            .any(|reference| reference.reference == annotation && reference.name == "Int")
    });
    let span = parsed
        .arena
        .node(candidate.declaration)
        .expect("parsed function declaration is in its arena")
        .span;
    if candidate.body.is_none() || !candidate.parameters.is_empty() || !has_int_return {
        return EntryPointReport {
            entry_point: None,
            diagnostics: vec![EntryPointDiagnostic::source(
                EntryPointDiagnosticKind::InvalidEntryPointSignature,
                span,
            )],
        };
    }

    EntryPointReport {
        entry_point: Some(EntryPoint {
            declaration: candidate.declaration,
            source_span: span,
        }),
        diagnostics: Vec::new(),
    }
}

pub fn check_straight_line_returns(parsed: &ParseOutput) -> ReturnPathReport {
    let mut diagnostics = Vec::new();
    for function in &parsed.function_declarations {
        let has_explicit_return = function.return_annotation.is_some();
        let Some(body) = function.body else {
            continue;
        };
        if !has_explicit_return {
            continue;
        }

        let direct_returns: Vec<_> = parsed
            .return_statements
            .iter()
            .filter(|returned| returned.function == function.declaration && returned.block == body)
            .collect();
        let Some((first, later)) = direct_returns.split_first() else {
            diagnostics.push(ReturnPathDiagnostic::new(
                ReturnPathDiagnosticKind::MissingReturn,
                function.declaration,
            ));
            continue;
        };
        if direct_returns
            .iter()
            .all(|returned| returned.value.is_none())
        {
            diagnostics.push(ReturnPathDiagnostic::new(
                ReturnPathDiagnosticKind::MissingReturn,
                function.declaration,
            ));
            continue;
        }
        let _ = first;
        diagnostics.extend(later.iter().map(|returned| {
            ReturnPathDiagnostic::new(
                ReturnPathDiagnosticKind::UnreachableReturn,
                returned.statement,
            )
        }));
    }
    ReturnPathReport { diagnostics }
}

pub fn check_return_expression_types(
    parsed: &ParseOutput,
    signatures: &[FunctionSignature],
    expression_types: &[ExpressionType],
) -> ReturnTypeReport {
    check_return_expression_types_with_arena(parsed, signatures, expression_types, None)
}

pub fn check_return_expression_types_with_arena(
    parsed: &ParseOutput,
    signatures: &[FunctionSignature],
    expression_types: &[ExpressionType],
    arena: Option<&TypeArena>,
) -> ReturnTypeReport {
    let mut diagnostics = Vec::new();

    for returned in &parsed.return_statements {
        let Some(expected) = signatures
            .iter()
            .find(|signature| signature.declaration == returned.function)
            .map(|signature| signature.return_type)
        else {
            continue;
        };
        let Some(value) = returned.value else {
            let is_void = arena.is_some_and(|arena| {
                matches!(
                    arena.get(expected).map(|record| record.kind()),
                    Some(TypeKind::Primitive(PrimitiveType::Void))
                )
            });
            if !is_void {
                let span = parsed
                    .arena
                    .node(returned.statement)
                    .map(|node| node.span)
                    .unwrap_or_else(|| parsed.arena.node(returned.function).unwrap().span);
                diagnostics.push(ReturnTypeDiagnostic::new(
                    ReturnTypeDiagnosticKind::ReturnTypeMismatch,
                    span,
                ));
            }
            continue;
        };
        let Some(actual) = expression_types
            .iter()
            .find(|typed| typed.expression == value)
            .map(|typed| typed.ty)
        else {
            continue;
        };
        let compatible = actual == expected
            || arena.is_some_and(|arena| {
                match (
                    arena.get(actual).map(|record| record.kind()),
                    arena.get(expected).map(|record| record.kind()),
                ) {
                    (
                        Some(TypeKind::Nominal(actual)),
                        Some(TypeKind::GenericInstance(expected)),
                    ) => actual == expected.declaration(),
                    (
                        Some(TypeKind::GenericInstance(actual)),
                        Some(TypeKind::GenericInstance(expected)),
                    ) => actual.declaration() == expected.declaration(),
                    (
                        Some(TypeKind::DynamicArray(actual)),
                        Some(TypeKind::DynamicArray(expected)),
                    ) => {
                        actual.element() == expected.element()
                            || matches!(
                                arena.get(expected.element()).map(|record| record.kind()),
                                Some(TypeKind::GenericParameter(_))
                            )
                    }
                    _ => false,
                }
            });
        if !compatible {
            let span = parsed.arena.node(value).expect("parsed return value").span;
            diagnostics.push(ReturnTypeDiagnostic::new(
                ReturnTypeDiagnosticKind::ReturnTypeMismatch,
                span,
            ));
        }
    }

    ReturnTypeReport { diagnostics }
}

pub fn check_unsupported_executable_forms(parsed: &ParseOutput) -> UnsupportedExecutableFormReport {
    let specific_form_spans: Vec<_> = parsed
        .arena
        .nodes()
        .iter()
        .filter(|node| {
            let dynamic_generic = node.kind == AstNodeKind::GenericArgument
                && parsed.type_name_references.iter().any(|reference| {
                    reference.name == "Array" && reference.generic_arguments.contains(&node.id)
                });
            !dynamic_generic
                && matches!(
                    node.kind,
                    AstNodeKind::BinaryExpression
                        | AstNodeKind::UnaryExpression
                        | AstNodeKind::CallExpression
                        | AstNodeKind::MemberExpression
                )
        })
        .map(|node| node.span)
        .collect();
    let mut candidates: Vec<_> = parsed
        .arena
        .nodes()
        .iter()
        .filter(|node| {
            matches!(
                node.kind,
                AstNodeKind::ImportDeclaration
                    | AstNodeKind::StructDeclaration
                    | AstNodeKind::NullableType
                    | AstNodeKind::GroupedType
            )
        })
        .map(|node| node.span)
        .collect();

    candidates.extend(
        parsed
            .literal_expressions
            .iter()
            .filter(|literal| matches!(literal.kind, ParsedLiteralKind::Null))
            .map(|literal| literal.span),
    );
    candidates.extend(
        parsed
            .type_name_references
            .iter()
            .filter(|reference| {
                reference.name != "Array"
                    && !matches!(
                        reference.name.as_str(),
                        "Bool" | "Int" | "String" | "Float" | "Byte" | "Channel" | "Task"
                    )
                    && !parsed
                        .generic_parameters
                        .iter()
                        .any(|parameter| parameter.name == reference.name)
                    && !parsed
                        .class_declarations
                        .iter()
                        .any(|class| class.name == reference.name)
            })
            .map(|reference| reference.name_span),
    );
    candidates.extend(
        parsed
            .function_declarations
            .iter()
            .filter(|function| function.body.is_none() && function.owner.is_none())
            .filter_map(|function| {
                parsed
                    .arena
                    .node(function.declaration)
                    .map(|node| node.span)
            }),
    );
    candidates.extend(
        parsed
            .local_declarations
            .iter()
            .filter(|declaration| declaration.initializer.is_none())
            .filter_map(|declaration| {
                parsed
                    .arena
                    .node(declaration.declaration)
                    .map(|node| node.span)
            }),
    );
    candidates.extend(
        parsed
            .return_statements
            .iter()
            .filter(|returned| returned.value.is_none())
            .filter_map(|returned| parsed.arena.node(returned.statement).map(|node| node.span)),
    );

    candidates.sort_by(|left, right| {
        left.file()
            .index()
            .cmp(&right.file().index())
            .then(left.start().cmp(&right.start()))
            .then(right.end().cmp(&left.end()))
    });
    candidates.dedup();

    let mut diagnostics = Vec::new();
    for span in candidates {
        if specific_form_spans
            .iter()
            .any(|specific| unsupported_form_span_contains(*specific, span))
        {
            continue;
        }
        if diagnostics
            .iter()
            .any(|diagnostic: &UnsupportedExecutableFormDiagnostic| {
                unsupported_form_span_contains(diagnostic.span, span)
            })
        {
            continue;
        }
        diagnostics.push(UnsupportedExecutableFormDiagnostic::new(span));
    }

    UnsupportedExecutableFormReport { diagnostics }
}

pub(crate) fn unsupported_form_span_contains(outer: ByteSpan, inner: ByteSpan) -> bool {
    outer.file() == inner.file()
        && outer.start() <= inner.start()
        && inner.end() <= outer.end()
        && outer != inner
}
