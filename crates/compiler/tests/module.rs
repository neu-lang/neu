use compiler::ast::{AstNodeId, AstNodeKind};
use compiler::module::{
    DeclarationVisibility, ModuleDiagnosticKind, ModuleMetadata, ModuleName,
    PackageGraphDiagnosticKind, PackageNamespace, VirtualPackageGraph, VirtualSource,
    VisibilityCategory, VisibilityOrigin,
};
use compiler::source::{SourceDatabase, SourceFileId};

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
    let first = sources.add_file("one/path/app.neu", "package app");
    let second = sources.add_file("different/path/app.neu", "package app");

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

#[test]
fn virtual_package_graph_loads_direct_directory_members_and_aliases() {
    let graph = VirtualPackageGraph::build(
        "src/main.neu",
        [
            VirtualSource::new(
                "src/main.neu",
                "import \"./math\" as arithmetic\nfunc main();",
            ),
            VirtualSource::new("src/math/add.neu", "package math\nfunc add();"),
            VirtualSource::new("src/math/sub.neu", "package math\nfunc sub();"),
            VirtualSource::new("src/math/nested/ignored.neu", "func ignored();"),
        ],
    )
    .unwrap();

    let math = graph
        .packages()
        .iter()
        .find(|package| package.directory == std::path::Path::new("src/math"))
        .unwrap();
    assert_eq!(math.identity, "math");
    assert_eq!(math.files.len(), 2);
    assert_eq!(
        graph
            .files()
            .iter()
            .map(|file| file.id.index())
            .collect::<Vec<_>>(),
        vec![0, 1, 3]
    );
}

#[test]
fn virtual_package_graph_rejects_file_imports_and_cycles() {
    let file_import = VirtualPackageGraph::build(
        "src/main.neu",
        [
            VirtualSource::new("src/main.neu", "import \"./math/add.neu\"\nfunc main();"),
            VirtualSource::new("src/math/add.neu", "func add();"),
        ],
    )
    .unwrap_err();
    assert!(
        file_import
            .iter()
            .any(|diagnostic| { diagnostic.kind == PackageGraphDiagnosticKind::FileImport })
    );

    let cycle = VirtualPackageGraph::build(
        "src/main.neu",
        [
            VirtualSource::new("src/main.neu", "import \"./a\"\nfunc main();"),
            VirtualSource::new("src/a/a.neu", "import \"../b\"\nfunc a();"),
            VirtualSource::new("src/b/b.neu", "import \"../a\"\nfunc b();"),
        ],
    )
    .unwrap_err();
    assert!(
        cycle
            .iter()
            .any(|diagnostic| { diagnostic.kind == PackageGraphDiagnosticKind::ImportCycle })
    );
}

#[test]
fn virtual_package_graph_rejects_private_imported_declarations() {
    let diagnostics = VirtualPackageGraph::build(
        "src/main.neu",
        [
            VirtualSource::new(
                "src/main.neu",
                "import \"./math\" as arithmetic\nfunc main(): Int { return arithmetic.secret() }",
            ),
            VirtualSource::new(
                "src/math/math.neu",
                "package math\nprivate func secret(): Int { return 1 }",
            ),
        ],
    )
    .unwrap_err();
    assert!(
        diagnostics.iter().any(|diagnostic| {
            diagnostic.kind == PackageGraphDiagnosticKind::InaccessibleImport
        })
    );
}

#[test]
fn visibility_metadata_represents_explicit_categories() {
    let public = DeclarationVisibility::explicit(
        AstNodeId::from_raw(10),
        AstNodeKind::FunctionDeclaration,
        VisibilityCategory::Public,
    )
    .unwrap();
    let internal = DeclarationVisibility::explicit(
        AstNodeId::from_raw(11),
        AstNodeKind::StructDeclaration,
        VisibilityCategory::Internal,
    )
    .unwrap();
    let private = DeclarationVisibility::explicit(
        AstNodeId::from_raw(12),
        AstNodeKind::InterfaceDeclaration,
        VisibilityCategory::Private,
    )
    .unwrap();

    assert_eq!(public.category(), VisibilityCategory::Public);
    assert_eq!(internal.category(), VisibilityCategory::Internal);
    assert_eq!(private.category(), VisibilityCategory::Private);
    assert_eq!(public.origin(), VisibilityOrigin::Explicit);
}

#[test]
fn default_visibility_is_public_and_defaulted() {
    let visibility = DeclarationVisibility::default_public(
        AstNodeId::from_raw(13),
        AstNodeKind::EnumDeclaration,
    )
    .unwrap();

    assert_eq!(visibility.category(), VisibilityCategory::Public);
    assert_eq!(visibility.origin(), VisibilityOrigin::Defaulted);
}

#[test]
fn visibility_metadata_supports_protected_and_public_default() {
    let protected = DeclarationVisibility::explicit(
        AstNodeId::from_raw(14),
        AstNodeKind::FunctionDeclaration,
        VisibilityCategory::Protected,
    )
    .unwrap();
    let defaulted = DeclarationVisibility::default_public(
        AstNodeId::from_raw(15),
        AstNodeKind::FunctionDeclaration,
    )
    .unwrap();

    assert_eq!(protected.category(), VisibilityCategory::Protected);
    assert_eq!(defaulted.category(), VisibilityCategory::Public);
    assert_eq!(defaulted.origin(), VisibilityOrigin::Defaulted);
}

#[test]
fn module_metadata_preserves_declaration_visibility_records() {
    let declaration = AstNodeId::from_raw(21);
    let visibility = DeclarationVisibility::explicit(
        declaration,
        AstNodeKind::FunctionDeclaration,
        VisibilityCategory::Private,
    )
    .unwrap();

    let metadata = ModuleMetadata::with_packages_and_visibility(
        ModuleName::parse("demo").unwrap(),
        [(SourceFileId::from_raw(0), PackageNamespace::root())],
        [visibility.clone()],
    )
    .unwrap();

    assert_eq!(metadata.visibility(), [visibility]);
    assert_eq!(metadata.visibility()[0].declaration(), declaration);
}

#[test]
fn visibility_metadata_does_not_attach_to_package_or_import_nodes() {
    for kind in [
        AstNodeKind::PackageDeclaration,
        AstNodeKind::ImportDeclaration,
    ] {
        assert_eq!(
            DeclarationVisibility::explicit(
                AstNodeId::from_raw(30),
                kind,
                VisibilityCategory::Public,
            )
            .unwrap_err()
            .kind,
            ModuleDiagnosticKind::UnsupportedVisibilityCategory
        );
    }
}
