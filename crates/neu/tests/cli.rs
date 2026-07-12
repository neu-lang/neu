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
fn build_compiles_the_repository_manifest_and_produces_an_executable() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = root.join("target/neu-cli-test");
    let _ = std::fs::remove_file(&output);
    let status = Command::new(env!("CARGO_BIN_EXE_neu"))
        .args(["build", root.join("neu.json").to_str().unwrap(), "--output"])
        .arg(&output)
        .status()
        .unwrap();
    assert!(status.success());
    assert_eq!(Command::new(&output).status().unwrap().code(), Some(7));
    let _ = std::fs::remove_file(output);
}
