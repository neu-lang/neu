use compiler::manifest::{ManifestDiagnosticKind, ProjectManifest};

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
    assert_eq!(manifest.entrypoint().to_string_lossy(), "src/main.neu");
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
