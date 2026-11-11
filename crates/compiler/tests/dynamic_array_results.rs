//! ADR-0138 (growth results) and ADR-0139 (move-out removal) on `T[]`.
//!
//! The results use `Result` from the real `stdlib/core/result.neu`, included
//! without its test functions, and `Error` from ADR-0135, declared here until
//! `stdlib/core` provides it. Result types are inferred because the compiler does
//! not accept `Unit` as a written type yet.

use std::{fs, process::Command};

use compiler::{
    driver::{SourceDriverOptions, compile_source_to_executable},
    module::{ModuleName, PackageNamespace},
    source::SourceFileId,
};

const ERROR_TYPES: &str = r#"
enum ErrorKind {
    IndexOutOfBounds,
    AllocationFailed,
}

interface Error {
    func kind(): ErrorKind;
}

class IndexOutOfBoundsError: Error {
    func kind(): ErrorKind {
        return ErrorKind.IndexOutOfBounds;
    }
}

class AllocationFailedError: Error {
    func kind(): ErrorKind {
        return ErrorKind.AllocationFailed;
    }
}
"#;

/// The real `stdlib/core/result.neu`, without its test functions.
fn result_core() -> &'static str {
    let source = include_str!("../../../stdlib/core/result.neu");
    source
        .split("public test func")
        .next()
        .expect("Result precedes its test functions")
}

/// The shared prelude: `Result`, the error types, and the test body.
fn program(body: &str) -> String {
    format!("{}\n{ERROR_TYPES}\n{body}", result_core())
}

/// Compiles and runs `source`. Returns the exit code, or the compile error text.
fn compile_and_run(name: &str, source: &str) -> Result<Option<i32>, String> {
    let workspace = std::env::temp_dir().join(format!(
        "neu-dynamic-array-results-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let compiled = compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(137_300),
            ModuleName::parse(name).unwrap(),
            PackageNamespace::root(),
            &executable,
        ),
    );
    let outcome = match compiled {
        Ok(_) => Ok(Command::new(&executable).status().unwrap().code()),
        Err(error) => Err(format!("{error:?}")),
    };
    let _ = fs::remove_dir_all(workspace);
    outcome
}

/// Asserts that `source` is rejected by a semantic rule. A parser rejection fails
/// here, because the rule under test would not have run.
fn assert_semantic_rejection(name: &str, source: &str) {
    match compile_and_run(name, source) {
        Ok(code) => panic!("expected a rejection, but the program ran with {code:?}"),
        Err(error) => assert!(
            !error.contains("ParseDiagnostics"),
            "rejected by the parser instead of the rule under test: {error}"
        ),
    }
}

#[test]
fn add_with_room_returns_ok_and_stores_the_element() {
    // ADR-0138 decision 1. Fails today: add returns Unit, not a Result.
    let source = program(
        r#"
func main(): Int {
    var values: Int[] = new Int[];
    val pushed = values.add(4);
    return when (pushed) {
        Result.Ok(_) -> values.size()
        Result.Err(_) -> 0
    };
}
"#,
    );
    assert_eq!(compile_and_run("array.add.ok", &source), Ok(Some(1)));
}

#[test]
fn indexed_add_outside_the_sequence_returns_an_index_error() {
    // ADR-0138 decision 5: an index outside 0..size is Err, not a trap. Fails today.
    let source = program(
        r#"
func main(): Int {
    var values: Int[] = new Int[];
    var index: Int = 3;
    val placed = values.add(1, index);
    return when (placed) {
        Result.Ok(_) -> 0
        Result.Err(error) -> when (error.kind()) {
            ErrorKind.IndexOutOfBounds -> 7
            ErrorKind.AllocationFailed -> 8
        }
    };
}
"#,
    );
    assert_eq!(
        compile_and_run("array.add.index.error", &source),
        Ok(Some(7))
    );
}

#[test]
fn add_on_a_val_binding_is_rejected() {
    // ADR-0073, unchanged: val rejects mutating operations. Passes today.
    let source = program(
        r#"
func main(): Int {
    val values: Int[] = new Int[];
    val pushed = values.add(1);
    return 0;
}
"#,
    );
    assert_semantic_rejection("array.add.val", &source);
}

#[test]
fn discarded_add_result_is_rejected() {
    // ADR-0138 decision 4 and Q1: a Result must be handled. Fails today: the
    // discarded Unit is accepted.
    let source = program(
        r#"
func main(): Int {
    var values: Int[] = new Int[];
    values.add(1);
    return 0;
}
"#,
    );
    assert_semantic_rejection("array.add.discarded", &source);
}

#[test]
fn remove_in_range_moves_the_element_out_and_shifts_the_rest() {
    // ADR-0139 decisions 1 to 4. Fails today: remove returns Unit.
    let source = program(
        r#"
func main(): Int {
    var values: Int[] = new Int[];
    val first = values.add(10);
    val second = values.add(20);
    val removed = values.remove(0);
    val kept: Int = values.size();
    return when (removed) {
        Result.Ok(value) -> value * 10 + kept
        Result.Err(_) -> 0
    };
}
"#,
    );
    // The removed element is 10 and one element remains: 10 * 10 + 1.
    assert_eq!(
        compile_and_run("array.remove.in.range", &source),
        Ok(Some(101))
    );
}

#[test]
fn remove_outside_the_sequence_returns_an_index_error() {
    // ADR-0139 decision 3. Fails today.
    let source = program(
        r#"
func main(): Int {
    var values: Int[] = new Int[];
    var index: Int = 2;
    val removed = values.remove(index);
    return when (removed) {
        Result.Ok(_) -> 0
        Result.Err(error) -> when (error.kind()) {
            ErrorKind.IndexOutOfBounds -> 7
            ErrorKind.AllocationFailed -> 8
        }
    };
}
"#,
    );
    assert_eq!(
        compile_and_run("array.remove.out.of.range", &source),
        Ok(Some(7))
    );
}

#[test]
fn discarded_remove_result_is_rejected() {
    // ADR-0139 decision 6 and Q2. Fails today: the discarded Unit is accepted.
    let source = program(
        r#"
func main(): Int {
    var values: Int[] = new Int[];
    val pushed = values.add(1);
    values.remove(0);
    return 0;
}
"#,
    );
    assert_semantic_rejection("array.remove.discarded", &source);
}

#[test]
fn remove_with_a_statically_negative_index_is_a_compile_error() {
    // ADR-0139 Q1: a statically known invalid index is still diagnosed before
    // lowering. Passes today; it guards that behavior after the Result change.
    let source = program(
        r#"
func main(): Int {
    var values: Int[] = new Int[];
    val removed = values.remove(-1);
    return 0;
}
"#,
    );
    assert_semantic_rejection("array.remove.static.negative", &source);
}
