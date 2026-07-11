use std::{fs, path::PathBuf, process::Command};

use compiler::{
    driver::{SourceDriverOptions, compile_source_to_executable},
    module::{ModuleName, PackageNamespace},
    source::SourceFileId,
};
use target_lexicon::Triple;

#[test]
fn compiles_current_example_to_host_executable_with_exit_status_seven() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let source_path = repo_root.join("examples/current/bootstrap_backend_smoke.neu");
    let source = fs::read_to_string(&source_path).unwrap();
    let workspace = std::env::temp_dir().join(format!("neu-source-driver-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");

    let output = compile_source_to_executable(
        &source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(1000),
            ModuleName::parse("examples.current").unwrap(),
            PackageNamespace::parse("examples.current").unwrap(),
            Triple::host(),
            repo_root.join("target-packs"),
            &executable,
        ),
    )
    .unwrap();

    let status = Command::new(output).status().unwrap();
    assert_eq!(status.code(), Some(7));
    let _ = fs::remove_dir_all(workspace);
}
