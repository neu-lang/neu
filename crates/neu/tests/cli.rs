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

#[test]
fn build_failure_renders_located_diagnostics() {
    let root = std::env::temp_dir().join(format!("neu-cli-diagnostics-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::write(
        root.join("neu.json"),
        r#"{"name":"cli.diagnostics","entrypoint":"src/main.neu","srcs":["src/main.neu"],"dependencies":[]}"#,
    )
    .unwrap();
    std::fs::write(
        root.join("src/main.neu"),
        "public func main(): Int { return \"hi\"; }\n",
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_neu"))
        .args(["build", root.join("neu.json").to_str().unwrap()])
        .output()
        .unwrap();
    // Diagnostics show `/` separators on every platform.
    let stderr = String::from_utf8_lossy(&output.stderr).replace('\\', "/");
    assert_eq!(output.status.code(), Some(1));
    assert!(stderr.contains("error[return_type_mismatch]"), "{stderr}");
    assert!(stderr.contains("--> src/main.neu:1:34"), "{stderr}");
    assert!(!stderr.contains("ReturnTypeDiagnostics("), "{stderr}");
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn build_explains_unsupported_lambda_capture() {
    let root = std::env::temp_dir().join(format!("neu-cli-capture-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::write(
        root.join("neu.json"),
        r#"{"name":"cli.capture","entrypoint":"src/main.neu","srcs":["src/*.neu"],"dependencies":[]}"#,
    )
    .unwrap();
    std::fs::write(
        root.join("src/main.neu"),
        "package cli.capture\n\nclass Box(var value: Int) {\n}\n\nsuspend func main(): Int {\n    scope {\n        val b: Box = new Box(1);\n        val t1 = spawn { -> b.value };\n        val t2 = spawn { -> b.value };\n        return await(t1) + await(t2);\n    }\n    return 0;\n}\n",
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_neu"))
        .args(["build", root.to_str().unwrap()])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success());
    assert!(
        stderr.contains("closure capture not supported"),
        "stderr was: {stderr}"
    );
    assert!(stderr.contains("must be Send"), "stderr was: {stderr}");
    let _ = std::fs::remove_dir_all(root);
}
