//! Native test entries (ADR-0115): a normal return passes, a passing assert
//! continues with the next statement, and a `Bool` `when` can feed an assert.

use std::{fs, process::Command};

use compiler::{
    driver::{SourceDriverOptions, compile_source_to_test_executable},
    module::{ModuleName, PackageNamespace},
    parser::parse_source,
    source::SourceFileId,
};

/// Compiles the test named `t` in `source` and returns its exit status.
fn run_test(name: &str, source: &str) -> Option<i32> {
    let file = SourceFileId::from_raw(80000 + name.len());
    let parsed = parse_source(file, source);
    let declaration = parsed
        .function_declarations
        .iter()
        .find(|function| function.is_test)
        .expect("source declares a test")
        .declaration;
    let workspace =
        std::env::temp_dir().join(format!("neu-test-entry-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = compile_source_to_test_executable(
        source,
        SourceDriverOptions::new(
            file,
            ModuleName::parse("test.entry").unwrap(),
            PackageNamespace::root(),
            workspace.join("test"),
        ),
        declaration,
    )
    .expect("test should compile");
    let code = Command::new(executable).status().unwrap().code();
    let _ = fs::remove_dir_all(workspace);
    code
}

const ORDERING: &str = "enum Ordering {\n    Less,\n    Equal,\n    Greater\n}\n";

#[test]
fn empty_test_exits_with_status_zero() {
    assert_eq!(run_test("empty", "public test func t() {\n}\n"), Some(0));
}

#[test]
fn passing_assert_lets_the_next_statement_run() {
    // The second assert must run and fail. A test that returned after the
    // first assert would exit with status zero.
    let code = run_test(
        "continue",
        "public test func t() {\n    assert(true, \"first\");\n    assert(false, \"second\");\n}\n",
    );
    assert_ne!(code, Some(0));
}

#[test]
fn bool_when_bound_then_asserted_passes() {
    let source = format!(
        "{ORDERING}public test func t() {{\n    val l: Ordering = Ordering.Less;\n    val ok: Bool = when (l) {{\n        Ordering.Less -> true\n        Ordering.Equal -> false\n        Ordering.Greater -> false\n    }};\n    assert(ok, \"bound\");\n}}\n"
    );
    assert_eq!(run_test("bool_when_pass", &source), Some(0));
}

#[test]
fn bool_when_bound_then_failing_assert_fails_the_test() {
    let source = format!(
        "{ORDERING}public test func t() {{\n    val l: Ordering = Ordering.Equal;\n    val ok: Bool = when (l) {{\n        Ordering.Less -> true\n        Ordering.Equal -> false\n        Ordering.Greater -> false\n    }};\n    assert(ok, \"must fail\");\n}}\n"
    );
    assert_ne!(run_test("bool_when_fail", &source), Some(0));
}
