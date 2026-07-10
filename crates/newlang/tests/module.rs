use newlang::module::{ModuleDiagnosticKind, ModuleMetadata, ModuleName, PackageNamespace};
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

#[test]
fn package_namespaces_validate_adr0025_segments_and_root() {
    let root = PackageNamespace::root();
    let simple = PackageNamespace::parse("demo").unwrap();
    let dotted = PackageNamespace::parse("demo.core_1.Api").unwrap();

    assert!(root.is_root());
    assert_eq!(root.as_str(), "");
    assert_eq!(simple.as_str(), "demo");
    assert_eq!(dotted.as_str(), "demo.core_1.Api");
}

#[test]
fn invalid_package_namespaces_report_package_diagnostics() {
    for invalid in [
        ".demo",
        "demo.",
        "demo..core",
        "demo-core",
        "demo.1core",
        "demo.π",
    ] {
        assert_eq!(
            PackageNamespace::parse(invalid).unwrap_err().kind,
            ModuleDiagnosticKind::InvalidPackageNamespace,
            "{invalid} should be rejected as an invalid package namespace"
        );
    }
}

#[test]
fn module_metadata_preserves_package_namespace_per_source_file() {
    let first = SourceFileId::from_raw(0);
    let second = SourceFileId::from_raw(1);
    let package = PackageNamespace::parse("demo.core").unwrap();

    let metadata = ModuleMetadata::with_packages(
        ModuleName::parse("demo").unwrap(),
        [(first, package.clone()), (second, package.clone())],
    )
    .unwrap();

    assert_eq!(metadata.source_files(), [first, second]);
    assert_eq!(metadata.packages()[0].source_file(), first);
    assert_eq!(metadata.packages()[0].namespace(), &package);
    assert_eq!(metadata.packages()[1].source_file(), second);
    assert_eq!(metadata.packages()[1].namespace(), &package);
}

#[test]
fn package_namespace_does_not_change_module_identity() {
    let root_metadata = ModuleMetadata::with_packages(
        ModuleName::parse("demo").unwrap(),
        [(SourceFileId::from_raw(0), PackageNamespace::root())],
    )
    .unwrap();
    let nested_metadata = ModuleMetadata::with_packages(
        ModuleName::parse("demo").unwrap(),
        [(
            SourceFileId::from_raw(1),
            PackageNamespace::parse("other.namespace").unwrap(),
        )],
    )
    .unwrap();

    assert_eq!(root_metadata.module_id(), nested_metadata.module_id());
    assert_ne!(
        root_metadata.packages()[0].namespace(),
        nested_metadata.packages()[0].namespace()
    );
}
