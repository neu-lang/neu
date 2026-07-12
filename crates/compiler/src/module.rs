use crate::ast::{AstNodeId, AstNodeKind};
use crate::parser::parse_source;
use crate::source::SourceFileId;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ModuleName(String);

impl ModuleName {
    pub fn parse(input: &str) -> Result<Self, ModuleDiagnostic> {
        if input.is_empty() {
            return Err(ModuleDiagnostic::new(
                ModuleDiagnosticKind::MissingModuleIdentity,
            ));
        }

        if input.split('.').all(is_identifier_segment) {
            Ok(Self(input.to_owned()))
        } else {
            Err(ModuleDiagnostic::new(
                ModuleDiagnosticKind::InvalidModuleIdentity,
            ))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn deterministic_id(&self) -> &str {
        self.as_str()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PackageNamespace(String);

impl PackageNamespace {
    pub fn root() -> Self {
        Self(String::new())
    }

    pub fn parse(input: &str) -> Result<Self, ModuleDiagnostic> {
        if input.is_empty() {
            return Ok(Self::root());
        }

        if input.split('.').all(is_identifier_segment) {
            Ok(Self(input.to_owned()))
        } else {
            Err(ModuleDiagnostic::new(
                ModuleDiagnosticKind::InvalidPackageNamespace,
            ))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_root(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionSymbolIdentity {
    module: ModuleName,
    package: PackageNamespace,
    name: String,
}

impl FunctionSymbolIdentity {
    pub fn new(module: ModuleName, package: PackageNamespace, name: impl Into<String>) -> Self {
        Self {
            module,
            package,
            name: name.into(),
        }
    }

    pub fn module(&self) -> &ModuleName {
        &self.module
    }

    pub fn package(&self) -> &PackageNamespace {
        &self.package
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceFilePackage {
    source_file: SourceFileId,
    namespace: PackageNamespace,
}

impl SourceFilePackage {
    pub fn source_file(&self) -> SourceFileId {
        self.source_file
    }

    pub fn namespace(&self) -> &PackageNamespace {
        &self.namespace
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VisibilityCategory {
    Public,
    Internal,
    Private,
    Protected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VisibilityOrigin {
    Explicit,
    Defaulted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarationVisibility {
    declaration: AstNodeId,
    category: VisibilityCategory,
    origin: VisibilityOrigin,
}

impl DeclarationVisibility {
    pub fn explicit(
        declaration: AstNodeId,
        declaration_kind: AstNodeKind,
        category: VisibilityCategory,
    ) -> Result<Self, ModuleDiagnostic> {
        Self::new(
            declaration,
            declaration_kind,
            category,
            VisibilityOrigin::Explicit,
        )
    }

    pub fn default_internal(
        declaration: AstNodeId,
        declaration_kind: AstNodeKind,
    ) -> Result<Self, ModuleDiagnostic> {
        Self::new(
            declaration,
            declaration_kind,
            VisibilityCategory::Internal,
            VisibilityOrigin::Defaulted,
        )
    }

    pub fn default_public(
        declaration: AstNodeId,
        declaration_kind: AstNodeKind,
    ) -> Result<Self, ModuleDiagnostic> {
        Self::new(
            declaration,
            declaration_kind,
            VisibilityCategory::Public,
            VisibilityOrigin::Defaulted,
        )
    }

    pub fn declaration(&self) -> AstNodeId {
        self.declaration
    }

    pub fn category(&self) -> VisibilityCategory {
        self.category
    }

    pub fn origin(&self) -> VisibilityOrigin {
        self.origin
    }

    fn new(
        declaration: AstNodeId,
        declaration_kind: AstNodeKind,
        category: VisibilityCategory,
        origin: VisibilityOrigin,
    ) -> Result<Self, ModuleDiagnostic> {
        if accepts_visibility(declaration_kind) {
            Ok(Self {
                declaration,
                category,
                origin,
            })
        } else {
            Err(ModuleDiagnostic::new(
                ModuleDiagnosticKind::UnsupportedVisibilityCategory,
            ))
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModuleMetadata {
    name: ModuleName,
    source_files: Vec<SourceFileId>,
    packages: Vec<SourceFilePackage>,
    visibility: Vec<DeclarationVisibility>,
}

impl ModuleMetadata {
    pub fn new(
        name: ModuleName,
        source_files: impl IntoIterator<Item = SourceFileId>,
    ) -> Result<Self, ModuleDiagnostic> {
        let source_files: Vec<SourceFileId> = source_files.into_iter().collect();
        let packages = source_files
            .iter()
            .copied()
            .map(|source_file| SourceFilePackage {
                source_file,
                namespace: PackageNamespace::root(),
            })
            .collect();
        Ok(Self {
            name,
            source_files,
            packages,
            visibility: Vec::new(),
        })
    }

    pub fn with_packages(
        name: ModuleName,
        packages: impl IntoIterator<Item = (SourceFileId, PackageNamespace)>,
    ) -> Result<Self, ModuleDiagnostic> {
        let packages: Vec<_> = packages
            .into_iter()
            .map(|(source_file, namespace)| SourceFilePackage {
                source_file,
                namespace,
            })
            .collect();
        let source_files = packages.iter().map(|package| package.source_file).collect();
        Ok(Self {
            name,
            source_files,
            packages,
            visibility: Vec::new(),
        })
    }

    pub fn with_packages_and_visibility(
        name: ModuleName,
        packages: impl IntoIterator<Item = (SourceFileId, PackageNamespace)>,
        visibility: impl IntoIterator<Item = DeclarationVisibility>,
    ) -> Result<Self, ModuleDiagnostic> {
        let packages: Vec<_> = packages
            .into_iter()
            .map(|(source_file, namespace)| SourceFilePackage {
                source_file,
                namespace,
            })
            .collect();
        let source_files = packages.iter().map(|package| package.source_file).collect();
        Ok(Self {
            name,
            source_files,
            packages,
            visibility: visibility.into_iter().collect(),
        })
    }

    pub fn name(&self) -> &ModuleName {
        &self.name
    }

    pub fn module_id(&self) -> &str {
        self.name.deterministic_id()
    }

    pub fn source_files(&self) -> &[SourceFileId] {
        &self.source_files
    }

    pub fn packages(&self) -> &[SourceFilePackage] {
        &self.packages
    }

    pub fn visibility(&self) -> &[DeclarationVisibility] {
        &self.visibility
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModuleDiagnostic {
    pub kind: ModuleDiagnosticKind,
}

impl ModuleDiagnostic {
    fn new(kind: ModuleDiagnosticKind) -> Self {
        Self { kind }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModuleDiagnosticKind {
    MissingModuleIdentity,
    InvalidModuleIdentity,
    AmbiguousSourceModuleAssignment,
    InvalidPackageNamespace,
    UnsupportedVisibilityCategory,
    DuplicateVisibilityMetadata,
}

fn accepts_visibility(kind: AstNodeKind) -> bool {
    matches!(
        kind,
        AstNodeKind::FunctionDeclaration
            | AstNodeKind::StructDeclaration
            | AstNodeKind::EnumDeclaration
            | AstNodeKind::InterfaceDeclaration
    )
}

fn is_identifier_segment(segment: &str) -> bool {
    let mut chars = segment.chars();
    let Some(first) = chars.next() else {
        return false;
    };

    is_identifier_start(first) && chars.all(is_identifier_continue)
}

fn is_identifier_start(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

fn is_identifier_continue(ch: char) -> bool {
    is_identifier_start(ch) || ch.is_ascii_digit()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VirtualSource {
    path: PathBuf,
    source: String,
}

impl VirtualSource {
    pub fn new(path: impl Into<PathBuf>, source: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            source: source.into(),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PackageGraphDiagnosticKind {
    InvalidSourcePath,
    SourceOutsideRoot,
    SourceParseDiagnostics,
    MissingPackageDirectory,
    EmptyPackageDirectory,
    FileImport,
    ImportPathTraversal,
    MalformedAlias,
    DuplicateAlias,
    PackageHeaderDisagreement,
    DuplicatePackageIdentity,
    ImportCycle,
    InaccessibleImport,
    ImportQualifierCollision,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageGraphDiagnostic {
    pub kind: PackageGraphDiagnosticKind,
    pub path: PathBuf,
    pub detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VirtualPackageFile {
    pub id: SourceFileId,
    pub path: PathBuf,
    pub package: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VirtualPackage {
    pub directory: PathBuf,
    pub identity: String,
    pub files: Vec<VirtualPackageFile>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VirtualPackageGraph {
    root: PathBuf,
    entry: PathBuf,
    sources: Vec<VirtualSource>,
    files: Vec<VirtualPackageFile>,
    packages: Vec<VirtualPackage>,
}

impl VirtualPackageGraph {
    pub fn build(
        entry: impl Into<PathBuf>,
        sources: impl IntoIterator<Item = VirtualSource>,
    ) -> Result<Self, Vec<PackageGraphDiagnostic>> {
        let entry_input: PathBuf = entry.into();
        let entry = normalize_relative_path(&entry_input).map_err(|kind| {
            vec![PackageGraphDiagnostic {
                kind,
                path: entry_input,
                detail: "entry source path must be a normalized relative .neu path".to_owned(),
            }]
        })?;
        let root = entry
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_path_buf();
        let mut input = BTreeMap::new();
        let mut diagnostics = Vec::new();
        for source in sources {
            let path = match normalize_relative_path(source.path()) {
                Ok(path) if path.extension().and_then(|ext| ext.to_str()) == Some("neu") => path,
                Ok(path) => {
                    diagnostics.push(PackageGraphDiagnostic {
                        kind: PackageGraphDiagnosticKind::InvalidSourcePath,
                        path,
                        detail: "virtual source files must use the .neu extension".to_owned(),
                    });
                    continue;
                }
                Err(kind) => {
                    diagnostics.push(PackageGraphDiagnostic {
                        kind,
                        path: source.path().to_owned(),
                        detail: "source paths must be normalized and relative".to_owned(),
                    });
                    continue;
                }
            };
            if !path.starts_with(&root) && root != Path::new("") {
                diagnostics.push(PackageGraphDiagnostic {
                    kind: PackageGraphDiagnosticKind::SourceOutsideRoot,
                    path,
                    detail: "source is outside the entry source root".to_owned(),
                });
                continue;
            }
            if input
                .insert(
                    path.clone(),
                    VirtualSource::new(path.clone(), source.source()),
                )
                .is_some()
            {
                diagnostics.push(PackageGraphDiagnostic {
                    kind: PackageGraphDiagnosticKind::InvalidSourcePath,
                    path,
                    detail: "duplicate virtual source path".to_owned(),
                });
            }
        }
        if !input.contains_key(&entry) {
            diagnostics.push(PackageGraphDiagnostic {
                kind: PackageGraphDiagnosticKind::InvalidSourcePath,
                path: entry.clone(),
                detail: "entry source is not present".to_owned(),
            });
        }
        if !diagnostics.is_empty() {
            return Err(diagnostics);
        }

        let mut source_db = crate::source::SourceDatabase::new();
        let mut file_by_path = BTreeMap::new();
        let mut files = Vec::new();
        for (path, source) in &input {
            let id = source_db.add_file(path.to_string_lossy(), source.source());
            file_by_path.insert(path.clone(), id);
            files.push((path.clone(), source.clone(), id));
        }

        let mut package_dirs = BTreeSet::new();
        package_dirs.insert(root.clone());
        let mut queue = vec![entry.clone()];
        let mut visited = BTreeSet::new();
        let mut edges = BTreeMap::<PathBuf, Vec<PathBuf>>::new();
        while let Some(file_path) = queue.pop() {
            if !visited.insert(file_path.clone()) {
                continue;
            }
            let source = input.get(&file_path).expect("queued source exists");
            let parsed = parse_source(file_by_path[&file_path], source.source());
            if !parsed.lex_diagnostics.is_empty() || !parsed.diagnostics.is_empty() {
                diagnostics.push(PackageGraphDiagnostic {
                    kind: PackageGraphDiagnosticKind::SourceParseDiagnostics,
                    path: file_path.clone(),
                    detail: "source contains lexical or parser diagnostics".to_owned(),
                });
                continue;
            }
            let package_dir = file_path
                .parent()
                .unwrap_or_else(|| Path::new(""))
                .to_path_buf();
            package_dirs.insert(package_dir.clone());
            let same_package_private: BTreeSet<_> = input
                .keys()
                .filter(|path| path.parent() == Some(package_dir.as_path()) && *path != &file_path)
                .flat_map(|path| {
                    let parsed = parse_source(file_by_path[path], input[path].source());
                    parsed
                        .function_declarations
                        .into_iter()
                        .filter(|function| function.top_level && function.visibility == "private")
                        .map(|function| function.name)
                })
                .collect();
            for reference in &parsed.name_references {
                if !reference.name.contains('.') && same_package_private.contains(&reference.name) {
                    diagnostics.push(PackageGraphDiagnostic {
                        kind: PackageGraphDiagnosticKind::InaccessibleImport,
                        path: file_path.clone(),
                        detail: reference.name.clone(),
                    });
                }
            }
            let mut aliases = BTreeSet::new();
            let mut qualifiers = BTreeMap::<String, PathBuf>::new();
            let current_qualifier =
                package_identity_for_directory(&package_dir, &input, &file_by_path);
            for import in parsed.imports {
                let explicit_alias = import.alias.clone();
                let alias = explicit_alias.clone().unwrap_or_default();
                if alias.is_empty() || !is_identifier_segment(&alias) {
                    if explicit_alias.is_some() {
                        diagnostics.push(PackageGraphDiagnostic {
                            kind: PackageGraphDiagnosticKind::MalformedAlias,
                            path: file_path.clone(),
                            detail: alias.clone(),
                        });
                    }
                } else if !aliases.insert(alias.clone()) {
                    diagnostics.push(PackageGraphDiagnostic {
                        kind: PackageGraphDiagnosticKind::DuplicateAlias,
                        path: file_path.clone(),
                        detail: alias.clone(),
                    });
                }
                if !import.path.starts_with('.') {
                    diagnostics.push(PackageGraphDiagnostic {
                        kind: PackageGraphDiagnosticKind::FileImport,
                        path: file_path.clone(),
                        detail: import.path,
                    });
                    continue;
                }
                let Some(directory) = resolve_import_directory(&file_path, &import.path, &root)
                else {
                    diagnostics.push(PackageGraphDiagnostic {
                        kind: PackageGraphDiagnosticKind::ImportPathTraversal,
                        path: file_path.clone(),
                        detail: import.path,
                    });
                    continue;
                };
                let qualifier = explicit_alias.unwrap_or_else(|| {
                    package_qualifier_for_directory(&directory, &input, &file_by_path)
                });
                if aliases.contains(&qualifier) && qualifier != alias {
                    diagnostics.push(PackageGraphDiagnostic {
                        kind: PackageGraphDiagnosticKind::ImportQualifierCollision,
                        path: file_path.clone(),
                        detail: format!("add an explicit alias for {qualifier}"),
                    });
                }
                if qualifier == current_qualifier
                    || parsed
                        .declaration_names
                        .iter()
                        .any(|declaration| declaration.name == qualifier)
                {
                    diagnostics.push(PackageGraphDiagnostic {
                        kind: PackageGraphDiagnosticKind::ImportQualifierCollision,
                        path: file_path.clone(),
                        detail: format!("qualifier {qualifier} conflicts with a local name"),
                    });
                }
                if let Some(previous) = qualifiers.insert(qualifier.clone(), directory.clone())
                    && previous != directory
                {
                    diagnostics.push(PackageGraphDiagnostic {
                        kind: PackageGraphDiagnosticKind::ImportQualifierCollision,
                        path: file_path.clone(),
                        detail: format!(
                            "qualifier {qualifier} names multiple packages; add an explicit alias"
                        ),
                    });
                }
                let imported_names: BTreeSet<_> = input
                    .keys()
                    .filter(|path| path.parent() == Some(directory.as_path()))
                    .flat_map(|path| {
                        let id = file_by_path[path];
                        let parsed = parse_source(id, input[path].source());
                        parsed
                            .function_declarations
                            .into_iter()
                            .filter(|function| {
                                function.top_level && function.visibility != "public"
                            })
                            .map(|function| function.name)
                    })
                    .collect();
                let aliases_to_check = [alias.as_str()];
                for member in &parsed.member_expressions {
                    let Some(receiver) = parsed
                        .name_references
                        .iter()
                        .find(|reference| reference.reference == member.receiver)
                    else {
                        continue;
                    };
                    if aliases_to_check.contains(&receiver.name.as_str())
                        && imported_names.contains(&member.name)
                    {
                        diagnostics.push(PackageGraphDiagnostic {
                            kind: PackageGraphDiagnosticKind::InaccessibleImport,
                            path: file_path.clone(),
                            detail: member.name.clone(),
                        });
                    }
                }
                let members: Vec<_> = input
                    .keys()
                    .filter(|path| path.parent() == Some(directory.as_path()))
                    .cloned()
                    .collect();
                if members.is_empty() {
                    let kind = if input.keys().any(|path| path == &directory) {
                        PackageGraphDiagnosticKind::FileImport
                    } else if input.keys().any(|path| path.starts_with(&directory)) {
                        PackageGraphDiagnosticKind::EmptyPackageDirectory
                    } else {
                        PackageGraphDiagnosticKind::MissingPackageDirectory
                    };
                    diagnostics.push(PackageGraphDiagnostic {
                        kind,
                        path: file_path.clone(),
                        detail: directory.to_string_lossy().into_owned(),
                    });
                    continue;
                }
                package_dirs.insert(directory.clone());
                edges
                    .entry(package_dir.clone())
                    .or_default()
                    .push(directory.clone());
                for member in members {
                    queue.push(member);
                }
            }
        }

        let mut packages = Vec::new();
        let mut identities = BTreeMap::new();
        for directory in package_dirs {
            let members: Vec<_> = files
                .iter()
                .filter(|(path, _, _)| path.parent() == Some(directory.as_path()))
                .map(|(path, source, id)| (path.clone(), source.clone(), *id))
                .collect();
            if members.is_empty() {
                diagnostics.push(PackageGraphDiagnostic {
                    kind: PackageGraphDiagnosticKind::EmptyPackageDirectory,
                    path: directory.clone(),
                    detail: directory.to_string_lossy().into_owned(),
                });
                continue;
            }
            let mut headers = BTreeSet::new();
            for (_, source, id) in &members {
                let parsed = parse_source(*id, source.source());
                if let Some(header) = parsed.package_name {
                    headers.insert(header);
                }
            }
            if headers.len() > 1 {
                diagnostics.push(PackageGraphDiagnostic {
                    kind: PackageGraphDiagnosticKind::PackageHeaderDisagreement,
                    path: directory.clone(),
                    detail: headers.into_iter().collect::<Vec<_>>().join(","),
                });
                continue;
            }
            let identity = headers.into_iter().next().unwrap_or_else(|| {
                directory
                    .components()
                    .filter_map(|component| component.as_os_str().to_str())
                    .collect::<Vec<_>>()
                    .join(".")
            });
            if identities
                .insert(identity.clone(), directory.clone())
                .is_some()
            {
                diagnostics.push(PackageGraphDiagnostic {
                    kind: PackageGraphDiagnosticKind::DuplicatePackageIdentity,
                    path: directory.clone(),
                    detail: identity.clone(),
                });
            }
            packages.push(VirtualPackage {
                directory: directory.clone(),
                identity: identity.clone(),
                files: members
                    .into_iter()
                    .map(|(path, _, id)| VirtualPackageFile {
                        id,
                        path,
                        package: identity.clone(),
                    })
                    .collect(),
            });
        }

        if let Some(cycle) = find_package_cycle(&edges, &root) {
            diagnostics.push(PackageGraphDiagnostic {
                kind: PackageGraphDiagnosticKind::ImportCycle,
                path: cycle,
                detail: "package imports form a cycle".to_owned(),
            });
        }
        if !diagnostics.is_empty() {
            return Err(diagnostics);
        }
        let graph_files = packages
            .iter()
            .flat_map(|package| package.files.iter().cloned())
            .collect();
        Ok(Self {
            root,
            entry,
            sources: input.into_values().collect(),
            files: graph_files,
            packages,
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
    pub fn entry(&self) -> &Path {
        &self.entry
    }
    pub fn sources(&self) -> &[VirtualSource] {
        &self.sources
    }
    pub fn files(&self) -> &[VirtualPackageFile] {
        &self.files
    }
    pub fn packages(&self) -> &[VirtualPackage] {
        &self.packages
    }

    pub fn bootstrap_source(&self) -> String {
        let mut sources = Vec::new();
        for file in &self.files {
            if file.path == self.entry
                || sources
                    .iter()
                    .any(|(path, _): &(PathBuf, String)| path == &file.path)
            {
                continue;
            }
            let source = self
                .sources
                .iter()
                .find(|source| source.path() == file.path)
                .expect("graph file has virtual source");
            sources.push((file.path.clone(), source.source().to_owned()));
        }
        let entry_source = self
            .sources
            .iter()
            .find(|source| source.path() == self.entry)
            .expect("entry has virtual source")
            .source()
            .to_owned();
        sources.insert(0, (self.entry.clone(), entry_source));

        let mut aliases = BTreeSet::new();
        if let Some(entry) = sources.first() {
            let id = self
                .files
                .iter()
                .find(|file| file.path == entry.0)
                .map(|file| file.id)
                .expect("entry has source id");
            for import in parse_source(id, &entry.1).imports {
                aliases.insert(import.alias.unwrap_or_else(|| {
                    Path::new(&import.path)
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or_default()
                        .to_owned()
                }));
            }
        }

        sources
            .into_iter()
            .map(|(_, source)| {
                source
                    .lines()
                    .filter(|line| {
                        let trimmed = line.trim_start();
                        !trimmed.starts_with("package ") && !trimmed.starts_with("import ")
                    })
                    .map(|line| {
                        let mut line = line.to_owned();
                        for alias in &aliases {
                            line = line.replace(&format!("{alias}."), "");
                        }
                        line
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn normalize_relative_path(path: &Path) -> Result<PathBuf, PackageGraphDiagnosticKind> {
    if path.is_absolute() {
        return Err(PackageGraphDiagnosticKind::InvalidSourcePath);
    }
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(part) => normalized.push(part),
            Component::ParentDir => {
                if !normalized.pop() {
                    return Err(PackageGraphDiagnosticKind::ImportPathTraversal);
                }
            }
            Component::RootDir | Component::Prefix(_) => {
                return Err(PackageGraphDiagnosticKind::InvalidSourcePath);
            }
        }
    }
    Ok(normalized)
}

fn package_identity_for_directory(
    directory: &Path,
    input: &BTreeMap<PathBuf, VirtualSource>,
    file_by_path: &BTreeMap<PathBuf, SourceFileId>,
) -> String {
    let mut header = None;
    for path in input.keys().filter(|path| path.parent() == Some(directory)) {
        let parsed = parse_source(file_by_path[path], input[path].source());
        if let Some(package) = parsed.package_name {
            header = Some(package);
            break;
        }
    }
    header.unwrap_or_else(|| {
        directory
            .components()
            .filter_map(|component| component.as_os_str().to_str())
            .collect::<Vec<_>>()
            .join(".")
    })
}

fn package_qualifier_for_directory(
    directory: &Path,
    input: &BTreeMap<PathBuf, VirtualSource>,
    file_by_path: &BTreeMap<PathBuf, SourceFileId>,
) -> String {
    let mut header = None;
    for path in input.keys().filter(|path| path.parent() == Some(directory)) {
        let parsed = parse_source(file_by_path[path], input[path].source());
        if let Some(package) = parsed.package_name {
            header = Some(package);
            break;
        }
    }
    header.unwrap_or_else(|| {
        directory
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("root")
            .to_owned()
    })
}

fn resolve_import_directory(file: &Path, import: &str, root: &Path) -> Option<PathBuf> {
    let parent = file.parent().unwrap_or_else(|| Path::new(""));
    let path = normalize_relative_path(&parent.join(import)).ok()?;
    if root == Path::new("") || path.starts_with(root) {
        Some(path)
    } else {
        None
    }
}

fn find_package_cycle(edges: &BTreeMap<PathBuf, Vec<PathBuf>>, start: &Path) -> Option<PathBuf> {
    fn visit(
        node: &Path,
        edges: &BTreeMap<PathBuf, Vec<PathBuf>>,
        active: &mut BTreeSet<PathBuf>,
        done: &mut BTreeSet<PathBuf>,
    ) -> Option<PathBuf> {
        if active.contains(node) {
            return Some(node.to_path_buf());
        }
        if !done.insert(node.to_path_buf()) {
            return None;
        }
        active.insert(node.to_path_buf());
        for next in edges.get(node).into_iter().flatten() {
            if let Some(cycle) = visit(next, edges, active, done) {
                return Some(cycle);
            }
        }
        active.remove(node);
        None
    }
    visit(start, edges, &mut BTreeSet::new(), &mut BTreeSet::new())
}
