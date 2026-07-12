use std::{fs, path::PathBuf, process::Command};

use compiler::{
    driver::{SourceDriverOptions, compile_source_to_executable},
    parser::parse_source,
    source::SourceFileId,
};
use target_lexicon::Triple;

#[test]
fn parses_fixed_array_types_literals_and_indexing() {
    let parsed = parse_source(
        SourceFileId::from_raw(6300),
        "func read(): Int { val values: [Int; 3] = [1, 2, 3]; return values[1]; }",
    );
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);
}

#[test]
fn accepts_named_const_array_length_and_nested_arrays() {
    let parsed = parse_source(
        SourceFileId::from_raw(6301),
        "func read(): Int { const N: Int = 2; val values: [[Int; N]; 2] = [[1, 2], [3, 4]]; return values[1][0]; }",
    );
    assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);
}

#[test]
fn compiles_and_runs_fixed_array_program() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace = std::env::temp_dir().join(format!("neu-array-driver-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = "public func main(): Int { var values: [Int; 3] = [1, 2, 3]; values[1] = 4; return values[1] + values[2]; }";
    let output = compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(6302),
            compiler::module::ModuleName::parse("arrays").unwrap(),
            compiler::module::PackageNamespace::root(),
            Triple::host(),
            repo_root.join("target-packs"),
            &executable,
        ),
    )
    .unwrap();
    assert_eq!(Command::new(output).status().unwrap().code(), Some(7));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn rejects_array_length_and_immutable_index_mutation() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace = std::env::temp_dir().join(format!("neu-array-negative-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source =
        "public func main(): Int { val values: [Int; 2] = [1, 2, 3]; values[0] = 4; return 0; }";
    let error = compiler::driver::compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(6303),
            compiler::module::ModuleName::parse("arrays").unwrap(),
            compiler::module::PackageNamespace::root(),
            Triple::host(),
            repo_root.join("target-packs"),
            &executable,
        ),
    )
    .unwrap_err();
    let rendered = format!("{error:?}");
    assert!(rendered.contains("ArrayLiteralLengthMismatch"));
    assert!(rendered.contains("ImmutableArrayMutation"));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn dynamic_out_of_bounds_index_traps() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace = std::env::temp_dir().join(format!("neu-array-trap-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = "public func main(): Int { val values: [Int; 2] = [1, 2]; var index: Int = 2; return values[index]; }";
    let output = compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(6304),
            compiler::module::ModuleName::parse("arrays").unwrap(),
            compiler::module::PackageNamespace::root(),
            Triple::host(),
            repo_root.join("target-packs"),
            &executable,
        ),
    )
    .unwrap();
    assert!(!Command::new(output).status().unwrap().success());
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn compiles_fixed_array_parameters_and_returns() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace = std::env::temp_dir().join(format!("neu-array-abi-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = r#"
        public func identity(values: [Int; 2]): [Int; 2] {
            return values;
        }
        public func main(): Int {
            val values: [Int; 2] = [3, 4];
            val returned: [Int; 2] = identity(values);
            return returned[0] + returned[1];
        }
    "#;
    let output = compiler::driver::compile_source_to_executable(
        source,
        compiler::driver::SourceDriverOptions::new(
            SourceFileId::from_raw(6305),
            compiler::module::ModuleName::parse("arrays").unwrap(),
            compiler::module::PackageNamespace::root(),
            Triple::host(),
            repo_root.join("target-packs"),
            &executable,
        ),
    )
    .unwrap();
    assert_eq!(Command::new(output).status().unwrap().code(), Some(7));
    let _ = fs::remove_dir_all(workspace);
}
