use newlang::module::{ModuleDiagnosticKind, ModuleMetadata, ModuleName};
use newlang::source::{SourceDatabase, SourceFileId};

#[test]
fn module_names_validate_adr0025_identifier_segments() {
    let simple = ModuleName::parse("app").unwrap();
    let dotted = ModuleName::parse("app.core_1.Tools").unwrap();

    assert_eq!(simple.as_str(), "app");
    assert_eq!(simple.deterministic_id(), "app");
    assert_eq!(dotted.as_str(), "app.core_1.Tools");
    assert_eq!(dotted.deterministic_id(), "app.core_1.Tools");
}

#[test]
fn module_name_diagnostics_distinguish_missing_from_invalid() {
    assert_eq!(
        ModuleName::parse("").unwrap_err().kind,
        ModuleDiagnosticKind::MissingModuleIdentity
    );

    for invalid in [
        ".core",
        "core.",
        "core..api",
        "core-api",
        "core.1api",
        "core.π",
    ] {
        assert_eq!(
            ModuleName::parse(invalid).unwrap_err().kind,
            ModuleDiagnosticKind::InvalidModuleIdentity,
            "{invalid} should be rejected as an invalid module identity"
        );
    }
}

#[test]
fn module_metadata_preserves_explicit_name_and_ordered_source_files() {
    let name = ModuleName::parse("app.core").unwrap();
    let files = [
        SourceFileId::from_raw(2),
        SourceFileId::from_raw(0),
        SourceFileId::from_raw(7),
    ];

    let metadata = ModuleMetadata::new(name.clone(), files).unwrap();

    assert_eq!(metadata.name(), &name);
    assert_eq!(metadata.module_id(), "app.core");
    assert_eq!(metadata.source_files(), files);
}

#[test]
fn module_identity_does_not_depend_on_source_file_paths() {
    let mut sources = SourceDatabase::new();
    let first = sources.add_file("one/path/app.nl", "package app");
    let second = sources.add_file("different/path/app.nl", "package app");

    let first_metadata =
        ModuleMetadata::new(ModuleName::parse("app.core").unwrap(), [first]).unwrap();
    let second_metadata =
        ModuleMetadata::new(ModuleName::parse("app.core").unwrap(), [second]).unwrap();

    assert_eq!(first_metadata.module_id(), second_metadata.module_id());
    assert_ne!(
        sources.file(first).unwrap().path(),
        sources.file(second).unwrap().path()
    );
}
