use std::{fs, process::Command};

use compiler::{
    driver::{SourceDriverOptions, compile_source_to_executable},
    parser::parse_source,
    source::SourceFileId,
};

#[test]
fn parses_fixed_array_types_literals_and_indexing() {
    let parsed = parse_source(
        SourceFileId::from_raw(6300),
        "func read(): Int { val values: Int[3] = [1, 2, 3]; return values[1]; }",
    );
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);

    let legacy = parse_source(
        SourceFileId::from_raw(6306),
        "func read(): Int { val values: [Int; 3] = [1, 2, 3]; return values[1]; }",
    );
    assert!(
        legacy
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.kind
                == compiler::parser::DiagnosticKind::MalformedArrayType)
    );
}

#[test]
fn accepts_named_const_array_length_and_nested_arrays() {
    let parsed = parse_source(
        SourceFileId::from_raw(6301),
        "func read(): Int { const N: Int = 2; val values: Int[2][N] = [[1, 2], [3, 4]]; return values[1][0]; }",
    );
    assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);
}

#[test]
fn compiles_and_runs_fixed_array_program() {
    let workspace = std::env::temp_dir().join(format!("neu-array-driver-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = "public func main(): Int { var values: Int[3] = [1, 2, 3]; values[1] = 4; return values[1] + values[2]; }";
    let output = compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(6302),
            compiler::module::ModuleName::parse("arrays").unwrap(),
            compiler::module::PackageNamespace::root(),
            &executable,
        ),
    )
    .unwrap();
    assert_eq!(Command::new(output).status().unwrap().code(), Some(7));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn iterates_fixed_array_elements_in_order() {
    let workspace =
        std::env::temp_dir().join(format!("neu-array-iteration-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = "public func main(): Int { val values: Int[3] = [1, 2, 4]; var total: Int = 0; for (value in values) { total = total + value; } return total; }";
    let output = compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(6315),
            compiler::module::ModuleName::parse("arrays").unwrap(),
            compiler::module::PackageNamespace::root(),
            &executable,
        ),
    )
    .unwrap();
    assert_eq!(Command::new(output).status().unwrap().code(), Some(7));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn iterates_dynamic_array_elements() {
    let workspace = std::env::temp_dir().join(format!(
        "neu-dynamic-array-iteration-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = "func make(): Int[] { var values: Int[] = new Int[]; values.add(1); values.add(2); values.add(4); return values; } public func main(): Int { val values: Int[] = make(); var total: Int = 0; for value in values { total = total + value; } return total; }";
    let output = compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(6316),
            compiler::module::ModuleName::parse("arrays").unwrap(),
            compiler::module::PackageNamespace::root(),
            &executable,
        ),
    )
    .unwrap();
    assert_eq!(Command::new(output).status().unwrap().code(), Some(7));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn accepts_generic_dynamic_array_storage() {
    let workspace = std::env::temp_dir().join(format!("neu-generic-array-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = "func make<T>(value: T): T[] { var values: T[] = new T[]; values.add(value); return values; } public func main(): Int { return 0; }";
    compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(6320),
            compiler::module::ModuleName::parse("arrays").unwrap(),
            compiler::module::PackageNamespace::root(),
            &executable,
        ),
    )
    .expect("generic array storage should type-check");
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn loads_dynamic_array_elements_by_index() {
    let workspace =
        std::env::temp_dir().join(format!("neu-dynamic-array-index-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = "public func main(): Int { var values: Int[] = new Int[]; values.add(4); values.add(7); return values[1]; }";
    let output = compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(6321),
            compiler::module::ModuleName::parse("arrays").unwrap(),
            compiler::module::PackageNamespace::root(),
            &executable,
        ),
    )
    .unwrap();
    assert_eq!(Command::new(output).status().unwrap().code(), Some(7));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn loads_dynamic_array_elements_inside_control_flow() {
    let workspace = std::env::temp_dir().join(format!(
        "neu-dynamic-array-index-flow-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = "public func main(): Int { var values: Int[] = new Int[]; values.add(4); values.add(7); var result: Int = 0; if (true) { result = values[1]; } return result; }";
    let output = compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(6323),
            compiler::module::ModuleName::parse("arrays").unwrap(),
            compiler::module::PackageNamespace::root(),
            &executable,
        ),
    )
    .unwrap();
    assert_eq!(Command::new(output).status().unwrap().code(), Some(7));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn rejects_dynamic_array_structural_mutation_during_iteration() {
    let workspace = std::env::temp_dir().join(format!(
        "neu-array-iteration-negative-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = "public func main(): Int { var values: Int[] = new Int[]; for value in values { values.add(value); } return 0; }";
    let error = compiler::driver::compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(6317),
            compiler::module::ModuleName::parse("arrays").unwrap(),
            compiler::module::PackageNamespace::root(),
            &executable,
        ),
    )
    .unwrap_err();
    assert!(format!("{error:?}").contains("ArrayIterationStructuralMutation"));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn rejects_array_length_and_immutable_index_mutation() {
    let workspace = std::env::temp_dir().join(format!("neu-array-negative-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source =
        "public func main(): Int { val values: Int[2] = [1, 2, 3]; values[0] = 4; return 0; }";
    let error = compiler::driver::compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(6303),
            compiler::module::ModuleName::parse("arrays").unwrap(),
            compiler::module::PackageNamespace::root(),
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
    let workspace = std::env::temp_dir().join(format!("neu-array-trap-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = "public func main(): Int { val values: Int[2] = [1, 2]; var index: Int = 2; return values[index]; }";
    let output = compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(6304),
            compiler::module::ModuleName::parse("arrays").unwrap(),
            compiler::module::PackageNamespace::root(),
            &executable,
        ),
    )
    .unwrap();
    assert!(!Command::new(output).status().unwrap().success());
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn compiles_fixed_array_parameters_and_returns() {
    let workspace = std::env::temp_dir().join(format!("neu-array-abi-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = r#"
        public func identity(values: Int[2]): Int[2] {
            return values;
        }
        public func main(): Int {
            val values: Int[2] = [3, 4];
            val returned: Int[2] = identity(values);
            return returned[0] + returned[1];
        }
    "#;
    let output = compiler::driver::compile_source_to_executable(
        source,
        compiler::driver::SourceDriverOptions::new(
            SourceFileId::from_raw(6305),
            compiler::module::ModuleName::parse("arrays").unwrap(),
            compiler::module::PackageNamespace::root(),
            &executable,
        ),
    )
    .unwrap();
    assert_eq!(Command::new(output).status().unwrap().code(), Some(7));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn compiles_empty_dynamic_array_and_reads_size() {
    let workspace = std::env::temp_dir().join(format!("neu-dynamic-array-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = "public func main(): Int { var values: Int[] = new Int[]; return values.size(); }";
    let output = compiler::driver::compile_source_to_executable(
        source,
        compiler::driver::SourceDriverOptions::new(
            SourceFileId::from_raw(6307),
            compiler::module::ModuleName::parse("arrays").unwrap(),
            compiler::module::PackageNamespace::root(),
            &executable,
        ),
    )
    .unwrap();
    assert_eq!(Command::new(output).status().unwrap().code(), Some(0));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn appends_and_removes_dynamic_array_elements() {
    let workspace =
        std::env::temp_dir().join(format!("neu-dynamic-array-mutate-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = "public func main(): Int { var values: Int[] = new Int[]; values.add(9, 0); values.add(7); values.remove(0); return values.size(); }";
    let output = compiler::driver::compile_source_to_executable(
        source,
        compiler::driver::SourceDriverOptions::new(
            SourceFileId::from_raw(6310),
            compiler::module::ModuleName::parse("arrays").unwrap(),
            compiler::module::PackageNamespace::root(),
            &executable,
        ),
    )
    .unwrap();
    assert_eq!(Command::new(output).status().unwrap().code(), Some(1));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn compiles_fixed_arrays_with_nominal_elements_and_field_access() {
    let workspace = std::env::temp_dir().join(format!("neu-nominal-array-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = "class Counter(var value: Int) {} public func main(): Int { val values: Counter[2] = [new Counter(3), new Counter(4)]; return values[1].value; }";
    let output = compiler::driver::compile_source_to_executable(
        source,
        compiler::driver::SourceDriverOptions::new(
            SourceFileId::from_raw(6312),
            compiler::module::ModuleName::parse("arrays").unwrap(),
            compiler::module::PackageNamespace::root(),
            &executable,
        ),
    )
    .unwrap();
    assert_eq!(Command::new(output).status().unwrap().code(), Some(4));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn compiles_fixed_arrays_with_interface_elements() {
    let workspace =
        std::env::temp_dir().join(format!("neu-interface-array-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = "interface Answerable { func value(): Int; } class Answer: Answerable { func value(): Int { return 11; } } public func main(): Int { val values: Answerable[1] = [new Answer()]; return values[0].value(); }";
    let output = compiler::driver::compile_source_to_executable(
        source,
        compiler::driver::SourceDriverOptions::new(
            SourceFileId::from_raw(6313),
            compiler::module::ModuleName::parse("arrays").unwrap(),
            compiler::module::PackageNamespace::root(),
            &executable,
        ),
    )
    .unwrap();
    assert_eq!(Command::new(output).status().unwrap().code(), Some(11));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn passes_dynamic_arrays_through_parameters_and_returns() {
    let workspace =
        std::env::temp_dir().join(format!("neu-dynamic-array-abi-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = "func identity(values: Int[]): Int[] { return values; } public func main(): Int { var values: Int[] = new Int[]; values.add(7); val returned: Int[] = identity(values); return returned.size(); }";
    let output = compiler::driver::compile_source_to_executable(
        source,
        compiler::driver::SourceDriverOptions::new(
            SourceFileId::from_raw(6314),
            compiler::module::ModuleName::parse("arrays").unwrap(),
            compiler::module::PackageNamespace::root(),
            &executable,
        ),
    )
    .unwrap();
    assert_eq!(Command::new(output).status().unwrap().code(), Some(1));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn indexed_assignment_inside_a_block_writes_the_element() {
    let workspace =
        std::env::temp_dir().join(format!("neu-array-block-write-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = "public func main(): Int { var values: Int[3] = [1, 2, 3]; if (values[0] == 1) { values[1] = 9; } return values[1]; }";
    let output = compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(6320),
            compiler::module::ModuleName::parse("arrays").unwrap(),
            compiler::module::PackageNamespace::root(),
            &executable,
        ),
    )
    .unwrap();
    assert_eq!(Command::new(output).status().unwrap().code(), Some(9));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn indexed_assignment_inside_a_block_uses_a_variable_index() {
    let workspace =
        std::env::temp_dir().join(format!("neu-array-block-index-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = "public func main(): Int { var values: Int[3] = [1, 2, 3]; val i: Int = 2; if (true) { values[i] = 5; } return values[2] + values[0]; }";
    let output = compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(6321),
            compiler::module::ModuleName::parse("arrays").unwrap(),
            compiler::module::PackageNamespace::root(),
            &executable,
        ),
    )
    .unwrap();
    assert_eq!(Command::new(output).status().unwrap().code(), Some(6));
    let _ = fs::remove_dir_all(workspace);
}

fn dynamic_array_exit_status(name: &str, source: &str, file: usize) -> Option<i32> {
    let workspace = std::env::temp_dir().join(format!("neu-array-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let output = compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(file),
            compiler::module::ModuleName::parse("arrays").unwrap(),
            compiler::module::PackageNamespace::root(),
            &executable,
        ),
    )
    .unwrap();
    let code = Command::new(output).status().unwrap().code();
    let _ = fs::remove_dir_all(workspace);
    code
}

#[test]
fn dynamic_array_element_write_stores_the_element() {
    assert_eq!(
        dynamic_array_exit_status(
            "element-write",
            "public func main(): Int { var values: Int[] = new Int[]; values.add(4); values.add(7); values[1] = 9; return values[1]; }",
            6330,
        ),
        Some(9)
    );
}

#[test]
fn dynamic_array_element_write_inside_a_block_stores_the_element() {
    assert_eq!(
        dynamic_array_exit_status(
            "element-write-block",
            "public func main(): Int { var values: Int[] = new Int[]; values.add(4); values.add(7); if (values[0] == 4) { values[1] = 9; } return values[1]; }",
            6331,
        ),
        Some(9)
    );
}

#[test]
fn dynamic_array_element_write_out_of_bounds_traps() {
    let code = dynamic_array_exit_status(
        "element-write-trap",
        "public func main(): Int { var values: Int[] = new Int[]; values.add(4); values[1] = 9; return 7; }",
        6332,
    );
    assert_ne!(code, Some(7));
}

#[test]
fn dynamic_array_element_write_on_a_val_array_is_rejected() {
    let workspace =
        std::env::temp_dir().join(format!("neu-array-val-write-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let error = compile_source_to_executable(
        "public func main(): Int { val values: Int[] = new Int[]; values[0] = 9; return 7; }",
        SourceDriverOptions::new(
            SourceFileId::from_raw(6333),
            compiler::module::ModuleName::parse("arrays").unwrap(),
            compiler::module::PackageNamespace::root(),
            workspace.join("program"),
        ),
    )
    .unwrap_err();
    assert!(format!("{error:?}").contains("ImmutableArrayMutation"));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn dynamic_array_bool_element_write_stores_the_element() {
    assert_eq!(
        dynamic_array_exit_status(
            "bool-element-write",
            "public func main(): Int { var values: Bool[] = new Bool[]; values.add(false); values[0] = true; if (values[0]) { return 7; } return 1; }",
            6334,
        ),
        Some(7)
    );
}

#[test]
fn array_spelling_is_not_a_type() {
    let workspace = std::env::temp_dir().join(format!("neu-array-spelling-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let error = compile_source_to_executable(
        "public func main(): Int { var values: Array<Int> = new Int[]; return 7; }",
        SourceDriverOptions::new(
            SourceFileId::from_raw(6340),
            compiler::module::ModuleName::parse("arrays").unwrap(),
            compiler::module::PackageNamespace::root(),
            workspace.join("program"),
        ),
    )
    .unwrap_err();
    assert!(format!("{error:?}").contains("Array"), "{error:?}");
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn dynamic_array_is_an_enum_payload() {
    let workspace = std::env::temp_dir().join(format!("neu-array-payload-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = compile_source_to_executable(
        "enum Holder<T> { Storage(T[]), Empty } public func main(): Int { return 7; }",
        SourceDriverOptions::new(
            SourceFileId::from_raw(6341),
            compiler::module::ModuleName::parse("arrays").unwrap(),
            compiler::module::PackageNamespace::root(),
            workspace.join("program"),
        ),
    )
    .unwrap();
    assert_eq!(Command::new(executable).status().unwrap().code(), Some(7));
    let _ = fs::remove_dir_all(workspace);
}
