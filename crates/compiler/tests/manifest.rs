use compiler::{
    manifest::{ManifestDiagnosticKind, ProjectManifest},
    module::{VirtualPackageGraph, VirtualSource},
};

#[test]
fn parses_and_expands_a_manifest_source_set() {
    let manifest = ProjectManifest::parse(
        r#"{
            "name": "example.app",
            "description": "Example",
            "entrypoint": "src/main.neu",
            "srcs": ["src/**/*.neu"],
            "dependencies": []
        }"#,
    )
    .unwrap();

    assert_eq!(manifest.name(), "example.app");
    assert_eq!(
        manifest.entrypoint().unwrap().to_string_lossy(),
        "src/main.neu"
    );
}

#[test]
fn parses_a_library_manifest_without_an_entrypoint() {
    let manifest = ProjectManifest::parse(
        r#"{
            "name": "neu.stdlib",
            "srcs": ["core/**/*.neu"],
            "dependencies": []
        }"#,
    )
    .unwrap();

    assert!(manifest.is_library());
    assert!(manifest.entrypoint().is_none());
    assert_eq!(
        manifest.require_entrypoint().unwrap_err().kind(),
        ManifestDiagnosticKind::EntrypointRequired
    );
}

#[test]
fn rejects_unknown_manifest_fields() {
    let error = ProjectManifest::parse(
        r#"{"name":"app","entrypoint":"main.neu","srcs":["*.neu"],"extra":true}"#,
    )
    .unwrap_err();

    assert_eq!(error.kind(), ManifestDiagnosticKind::UnknownField);
}

#[test]
fn expands_sorted_deduplicated_neu_globs_and_requires_entrypoint() {
    let root = std::env::temp_dir().join(format!("neu-manifest-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("src/nested")).unwrap();
    std::fs::write(root.join("src/z.neu"), "").unwrap();
    std::fs::write(root.join("src/main.neu"), "").unwrap();
    std::fs::write(root.join("src/nested/a.neu"), "").unwrap();
    let manifest = ProjectManifest::parse(
        r#"{"name":"app","entrypoint":"src/main.neu","srcs":["src/**/*.neu","src/main.neu"]}"#,
    )
    .unwrap();

    let files = manifest.expand_sources(&root).unwrap();
    assert_eq!(
        files
            .iter()
            .map(|path| path
                .strip_prefix(&root)
                .unwrap()
                .to_string_lossy()
                .into_owned())
            .collect::<Vec<_>>(),
        vec!["src/main.neu", "src/nested/a.neu", "src/z.neu"]
    );
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn expands_library_sources_from_project_root_without_an_entrypoint() {
    let root = std::env::temp_dir().join(format!("neu-library-manifest-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("core/nested")).unwrap();
    std::fs::write(root.join("core/option.neu"), "").unwrap();
    std::fs::write(root.join("core/nested/result.neu"), "").unwrap();
    let manifest = ProjectManifest::parse(
        r#"{"name":"neu.stdlib","srcs":["core/**/*.neu"],"dependencies":[]}"#,
    )
    .unwrap();

    let sources = manifest.load_sources(&root).unwrap();
    assert_eq!(
        sources
            .iter()
            .map(|source| source.path().to_string_lossy().into_owned())
            .collect::<Vec<_>>(),
        vec!["core/nested/result.neu", "core/option.neu"]
    );
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn stdlib_manifest_includes_sibling_collections_sources() {
    let manifest_path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../stdlib/neu.json");
    let (manifest, root) = ProjectManifest::load(&manifest_path).unwrap();
    let sources = manifest.load_sources(root).unwrap();
    assert!(
        sources
            .iter()
            .any(|source| { source.path().to_string_lossy() == "collections/vector.neu" })
    );
}

#[test]
fn unified_stdlib_sources_form_core_and_collections_packages() {
    let manifest_path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../stdlib/neu.json");
    let (manifest, root) = ProjectManifest::load(&manifest_path).unwrap();
    let sources = manifest.load_sources(root).unwrap();
    let graph = VirtualPackageGraph::build_library(
        sources
            .into_iter()
            .map(|source| VirtualSource::new(source.path(), source.source())),
    )
    .unwrap();
    let packages = graph
        .packages()
        .iter()
        .map(|package| package.identity.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(packages, ["collections", "core"].into_iter().collect());
}
