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
pub struct ModuleMetadata {
    name: ModuleName,
    source_files: Vec<SourceFileId>,
}

impl ModuleMetadata {
    pub fn new(
        name: ModuleName,
        source_files: impl IntoIterator<Item = SourceFileId>,
    ) -> Result<Self, ModuleDiagnostic> {
        let source_files = source_files.into_iter().collect();
        Ok(Self { name, source_files })
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
