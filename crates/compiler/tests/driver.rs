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

#[test]
fn compiles_current_control_flow_and_primitive_examples() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    for (name, expected_status) in [
        ("control_flow", 7),
        ("primitive_values", 7),
        ("fixed_arrays", 7),
        ("strings", 7),
        ("classes", 7),
        ("class_methods", 9),
        ("interfaces", 11),
        ("virtual_dispatch", 13),
        ("dispatch_parameters", 10),
        ("overloads", 10),
        ("conditional_values", 10),
        ("dynamic_arrays", 1),
        ("nominal_arrays", 4),
        ("optional_semicolons", 4),
        ("enum_values", 7),
    ] {
        let source_path = repo_root.join(format!("examples/current/{name}.neu"));
        let source = fs::read_to_string(&source_path).unwrap();
        let workspace =
            std::env::temp_dir().join(format!("neu-example-driver-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&workspace);
        fs::create_dir_all(&workspace).unwrap();
        let executable = workspace.join("program");
        let output = compile_source_to_executable(
            &source,
            SourceDriverOptions::new(
                SourceFileId::from_raw(1002),
                ModuleName::parse("examples.current").unwrap(),
                PackageNamespace::parse("examples.current").unwrap(),
                Triple::host(),
                repo_root.join("target-packs"),
                &executable,
            ),
        )
        .unwrap_or_else(|error| panic!("example {name}: {error:?}"));
        let status = Command::new(output).status().unwrap();
        assert_eq!(status.code(), Some(expected_status), "example {name}");
        let _ = fs::remove_dir_all(workspace);
    }
}

#[test]
fn compiles_primitive_parameter_and_return_matrix() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace =
        std::env::temp_dir().join(format!("neu-primitive-matrix-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = r#"
        public func bool_identity(value: Bool): Bool { return value; }
        public func float_identity(value: Float): Float { return value; }
        public func byte_identity(value: Byte): Byte { return value; }
        public func unit_identity(value: Unit): Unit { return (); }
        public func main(): Int {
            bool_identity(true);
            float_identity(2.5);
            val byte: Byte = 7;
            byte_identity(byte);
            unit_identity(());
            return 0;
        }
    "#;
    let output = compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(9300),
            compiler::module::ModuleName::parse("matrix").unwrap(),
            compiler::module::PackageNamespace::root(),
            Triple::host(),
            repo_root.join("target-packs"),
            &executable,
        ),
    )
    .unwrap();
    assert_eq!(Command::new(output).status().unwrap().code(), Some(0));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn compiles_zero_payload_enum_values_through_typed_parameters_and_returns() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace = std::env::temp_dir().join(format!("neu-enum-driver-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = r#"
        enum Signal { Red, Green }
        func red(): Signal { return Signal.Red; }
        func select(value: Signal): Int { return 7; }
        public func main(): Int {
            val signal: Signal = Signal.Red;
            return select(signal);
        }
    "#;
    let output = compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(7900),
            ModuleName::parse("enum_values").unwrap(),
            PackageNamespace::root(),
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
fn resolves_exact_top_level_overloads_end_to_end() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace =
        std::env::temp_dir().join(format!("neu-overload-driver-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = r#"
        func select(value: Int): Int { return 3; }
        func select(value: Bool): Int { return 4; }
        public func main(): Int { return select(true); }
    "#;
    let output = compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(7600),
            ModuleName::parse("overloads").unwrap(),
            PackageNamespace::root(),
            Triple::host(),
            repo_root.join("target-packs"),
            &executable,
        ),
    )
    .unwrap();
    assert_eq!(Command::new(output).status().unwrap().code(), Some(4));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn rejects_ambiguous_and_missing_overloads_before_lowering() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    for (source, expected) in [
        (
            "func select(value: Int): Int { return 1; } public func main(): Int { return select(true); }",
            "ArgumentTypeMismatch",
        ),
        (
            "class Base {} interface Answer {} class Child: Base(), Answer { } func select(value: Base): Int { return 1; } func select(value: Answer): Int { return 2; } public func main(): Int { return select(new Child()); }",
            "AmbiguousOverload",
        ),
        (
            "func select(value: Int): Int { return 1; } func select(value: Int): Int { return 2; } public func main(): Int { return select(1); }",
            "DuplicateOverload",
        ),
    ] {
        let workspace =
            std::env::temp_dir().join(format!("neu-overload-negative-{}", std::process::id()));
        let _ = fs::remove_dir_all(&workspace);
        fs::create_dir_all(&workspace).unwrap();
        let error = compile_source_to_executable(
            source,
            SourceDriverOptions::new(
                SourceFileId::from_raw(7601),
                ModuleName::parse("overloads").unwrap(),
                PackageNamespace::root(),
                Triple::host(),
                repo_root.join("target-packs"),
                workspace.join("program"),
            ),
        )
        .unwrap_err();
        assert!(format!("{error:?}").contains(expected));
        let _ = fs::remove_dir_all(workspace);
    }
}

#[test]
fn compiles_value_producing_conditional_expression() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace = std::env::temp_dir().join(format!("neu-if-value-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let source = "public func main(): Int { val result: Int = if (true) { 10; } else { 20; }; return result; }";
    let output = compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(7700),
            ModuleName::parse("conditionals").unwrap(),
            PackageNamespace::root(),
            Triple::host(),
            repo_root.join("target-packs"),
            workspace.join("program"),
        ),
    )
    .unwrap();
    assert_eq!(Command::new(output).status().unwrap().code(), Some(10));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn value_conditionals_short_circuit_and_nest() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace =
        std::env::temp_dir().join(format!("neu-if-value-nested-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let source = "func zero(): Int { return 0; } public func main(): Int { val result: Int = if (true) { 10; } else { zero(); }; return result; }";
    let output = compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(7701),
            ModuleName::parse("conditionals").unwrap(),
            PackageNamespace::root(),
            Triple::host(),
            repo_root.join("target-packs"),
            workspace.join("program"),
        ),
    )
    .unwrap();
    assert_eq!(Command::new(output).status().unwrap().code(), Some(10));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn rejects_invalid_value_conditional_forms() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    for (source, expected) in [
        (
            "public func main(): Int { val result: Int = if (1) { 1; } else { 2; }; return result; }",
            "ConditionalConditionNotBool",
        ),
        (
            "public func main(): Int { val result: Int = if (true) { 1; }; return result; }",
            "ConditionalElseRequired",
        ),
        (
            "public func main(): Int { val result: Int = if (true) { 1; } else { false; }; return result; }",
            "ConditionalBranchTypeMismatch",
        ),
    ] {
        let workspace =
            std::env::temp_dir().join(format!("neu-if-value-negative-{}", std::process::id()));
        let _ = fs::remove_dir_all(&workspace);
        fs::create_dir_all(&workspace).unwrap();
        let error = compile_source_to_executable(
            source,
            SourceDriverOptions::new(
                SourceFileId::from_raw(7702),
                ModuleName::parse("conditionals").unwrap(),
                PackageNamespace::root(),
                Triple::host(),
                repo_root.join("target-packs"),
                workspace.join("program"),
            ),
        )
        .unwrap_err();
        assert!(format!("{error:?}").contains(expected));
        let _ = fs::remove_dir_all(workspace);
    }
}

#[test]
fn rejects_runtime_calls_in_const_initializers() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace = std::env::temp_dir().join(format!("neu-const-driver-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let error = compiler::driver::compile_source_to_executable(
        "func compute(): Int { return 1; } public func main(): Int { const value = compute(); return 7; }",
        SourceDriverOptions::new(
            SourceFileId::from_raw(1003),
            ModuleName::parse("consts").unwrap(),
            PackageNamespace::root(),
            Triple::host(),
            repo_root.join("target-packs"),
            &executable,
        ),
    )
    .unwrap_err();
    assert!(format!("{error:?}").contains("ConstInitializerNotConstant"));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn compiles_if_for_break_and_continue_to_host_executable() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace = std::env::temp_dir().join(format!("neu-control-driver-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let output = compile_source_to_executable(
        "public func main(): Int { for (index in 0..3) { if (index == 0) { continue; } else { if (index == 2) { break; } } } return 7; }",
        SourceDriverOptions::new(
            SourceFileId::from_raw(1001),
            ModuleName::parse("control").unwrap(),
            PackageNamespace::root(),
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
