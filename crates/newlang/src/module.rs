use crate::source::SourceFileId;

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModuleMetadata {
    name: ModuleName,
    source_files: Vec<SourceFileId>,
    packages: Vec<SourceFilePackage>,
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
