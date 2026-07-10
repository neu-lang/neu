use crate::ast::{AstNodeId, AstNodeKind};
use crate::source::SourceFileId;

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
