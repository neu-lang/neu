use std::process::Command;

#[test]
fn build_help_is_available() {
    let output = Command::new(env!("CARGO_BIN_EXE_neu"))
        .args(["build", "--help"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Build a Neu project"));
}

#[test]
fn test_lists_and_runs_test_only_project() {
    let root = std::env::temp_dir().join(format!("neu-cli-tests-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::write(
        root.join("neu.json"),
        r#"{"name":"cli.tests","srcs":["src/*.neu"],"dependencies":[]}"#,
    )
    .unwrap();
    std::fs::write(
        root.join("src/tests.neu"),
        "public test func option_some_is_some() { assert(true, \"expected Some\") }",
    )
    .unwrap();
    let manifest = root.join("neu.json");
    let listed = Command::new(env!("CARGO_BIN_EXE_neu"))
        .args(["test", manifest.to_str().unwrap(), "--list"])
        .output()
        .unwrap();
    assert!(listed.status.success());
    assert!(String::from_utf8_lossy(&listed.stdout).contains("cli.tests::option_some_is_some"));
    let status = Command::new(env!("CARGO_BIN_EXE_neu"))
        .args(["test", manifest.to_str().unwrap()])
        .status()
        .unwrap();
    assert!(status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_reports_failed_assertion_with_nonzero_status() {
    let root = std::env::temp_dir().join(format!("neu-cli-test-failure-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::write(
        root.join("neu.json"),
        r#"{"name":"cli.failures","srcs":["src/*.neu"],"dependencies":[]}"#,
    )
    .unwrap();
    std::fs::write(
        root.join("src/tests.neu"),
        "public test func fails() { assert(false, \"expected Some\") }",
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_neu"))
        .args(["test", root.join("neu.json").to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("expected Some"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn build_compiles_an_explicit_neu_project_manifest() {
    let root = std::env::temp_dir().join(format!("neu-cli-project-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::write(
        root.join("neu.json"),
        r#"{"name":"cli.smoke","entrypoint":"src/main.neu","srcs":["src/main.neu"],"dependencies":[]}"#,
    )
    .unwrap();
    std::fs::write(
        root.join("src/main.neu"),
        "public func main(): Int { return 7; }",
    )
    .unwrap();
    let manifest = root.join("neu.json");
    let output = root.join("target/neu-cli-test");
    let status = Command::new(env!("CARGO_BIN_EXE_neu"))
        .args(["build", manifest.to_str().unwrap(), "--output"])
        .arg(&output)
        .status()
        .unwrap();
    assert!(status.success());
    assert_eq!(Command::new(&output).status().unwrap().code(), Some(7));
    let _ = std::fs::remove_dir_all(root);
}
