//! Local type inference (ADR-0090) and single-expression lambda bodies
//! (SPEC, Lambda Bodies). Each program must compile and exit with the value
//! its `main` returns.

use std::{fs, process::Command};

use compiler::{
    driver::{
        SourceDriverOptions, compile_virtual_project_to_executable, validate_virtual_project,
    },
    module::{ModuleName, PackageNamespace, VirtualSource},
    source::SourceFileId,
};

fn run_exit_code(name: &str, main: &str) -> Option<i32> {
    let workspace =
        std::env::temp_dir().join(format!("neu-local-inference-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let project =
        validate_virtual_project("src/main.neu", [VirtualSource::new("src/main.neu", main)])
            .unwrap();
    let executable = compile_virtual_project_to_executable(
        &project,
        SourceDriverOptions::new(
            SourceFileId::from_raw(31600),
            ModuleName::parse("local.inference").unwrap(),
            PackageNamespace::root(),
            workspace.join("program"),
        ),
    )
    .unwrap();
    let code = Command::new(executable).status().unwrap().code();
    let _ = fs::remove_dir_all(workspace);
    code
}

#[test]
fn local_from_a_literal_infers_its_type() {
    assert_eq!(
        run_exit_code(
            "literal",
            "public func main(): Int {\n    val x = 5;\n    return x;\n}\n",
        ),
        Some(5)
    );
}

#[test]
fn local_from_a_call_infers_its_type() {
    assert_eq!(
        run_exit_code(
            "call",
            "func seven(): Int { return 7; }\npublic func main(): Int {\n    val x = seven();\n    return x;\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn local_from_an_expression_infers_its_type() {
    assert_eq!(
        run_exit_code(
            "expression",
            "public func main(): Int {\n    val x = 2 + 3;\n    return x;\n}\n",
        ),
        Some(5)
    );
}

#[test]
fn annotated_lambda_local_is_callable() {
    assert_eq!(
        run_exit_code(
            "annotated_lambda",
            "public func main(): Int {\n    val f: () -> Int = { -> 7 };\n    return f();\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn lambda_argument_is_callable_with_a_single_expression_body() {
    assert_eq!(
        run_exit_code(
            "lambda_argument",
            "func apply(g: () -> Int): Int { return g(); }\npublic func main(): Int {\n    return apply({ -> 7 });\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn unannotated_lambda_local_is_callable() {
    assert_eq!(
        run_exit_code(
            "unannotated_lambda",
            "public func main(): Int {\n    val f = { -> 7 };\n    return f();\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn unannotated_lambda_local_infers_its_return_type_from_the_body() {
    assert_eq!(
        run_exit_code(
            "inferred_return",
            "public func main(): Int {\n    val f = { x: Int -> x + 1 };\n    return f(6);\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn lambda_parameter_type_is_inferred_from_the_call_argument() {
    assert_eq!(
        run_exit_code(
            "argument_context",
            "func apply(g: (Int) -> Int, v: Int): Int { return g(v); }\npublic func main(): Int {\n    return apply({ x -> x + 2 }, 5);\n}\n",
        ),
        Some(7)
    );
}
