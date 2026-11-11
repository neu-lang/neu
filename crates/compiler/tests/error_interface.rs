//! ADR-0135: the stdlib `Error` interface, the closed `ErrorKind`, and the error
//! classes. The error types are declared in source until `stdlib/core` provides
//! them, so these tests exercise the language rules rather than the stdlib file.

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

/// Compiles and runs `source`. Returns the exit code, or the compile error text.
fn compile_and_run(name: &str, source: &str) -> Result<Option<i32>, String> {
    let workspace =
        std::env::temp_dir().join(format!("neu-error-interface-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let compiled = compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(137_100),
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

fn with_error_types(body: &str) -> String {
    format!("{ERROR_TYPES}\n{body}")
}

#[test]
fn class_implementing_error_reports_its_kind_through_the_interface() {
    // Passes today; it pins the upcast from a class to Error (ADR-0135 decision 4).
    let source = with_error_types(
        r#"
func main(): Int {
    val error: Error = new AllocationFailedError();
    return when (error.kind()) {
        ErrorKind.AllocationFailed -> 5
        ErrorKind.IndexOutOfBounds -> 6
    };
}
"#,
    );
    assert_eq!(
        compile_and_run("error.kind.through.interface", &source),
        Ok(Some(5))
    );
}

#[test]
fn enum_cannot_implement_the_error_interface() {
    // ADR-0079: enum declarations do not inherit classes or interfaces. A parser
    // rejection is acceptable here, so this test only checks that compilation fails.
    let source = with_error_types(
        r#"
enum BadError: Error {
    Oops,

    func kind(): ErrorKind {
        return ErrorKind.IndexOutOfBounds;
    }
}

func main(): Int {
    return 0;
}
"#,
    );
    assert!(compile_and_run("error.enum.rejected", &source).is_err());
}

#[test]
fn class_implementing_error_must_provide_kind() {
    // Passes today with MissingInterfaceMethod.
    let source = with_error_types(
        r#"
class Silent: Error {
}

func main(): Int {
    return 0;
}
"#,
    );
    assert_semantic_rejection("error.missing.kind", &source);
}

#[test]
fn when_over_error_kind_must_cover_every_case() {
    // ADR-0135 decision 8. Passes today with WhenNonExhaustive.
    let source = with_error_types(
        r#"
func main(): Int {
    val error: Error = new IndexOutOfBoundsError();
    return when (error.kind()) {
        ErrorKind.IndexOutOfBounds -> 1
    };
}
"#,
    );
    assert_semantic_rejection("error.kind.exhaustive", &source);
}

#[test]
fn result_can_carry_an_error_interface_value() {
    // ADR-0135 decision 5 and O4. Fails today: a class value is not upcast to the
    // interface type when it is passed as a generic enum payload (`Outcome.Bad`).
    // Passing an already-typed interface value works, so the gap is the upcast.
    let source = with_error_types(
        r#"
enum Outcome<T, E> {
    Good(T),
    Bad(E),
}

func main(): Int {
    val outcome: Outcome<Int, Error> = Outcome.Bad(new AllocationFailedError());
    return when (outcome) {
        Outcome.Good(value) -> value
        Outcome.Bad(error) -> when (error.kind()) {
            ErrorKind.AllocationFailed -> 8
            ErrorKind.IndexOutOfBounds -> 9
        }
    };
}
"#,
    );
    assert_eq!(
        compile_and_run("error.outcome.payload", &source),
        Ok(Some(8))
    );
}

#[test]
fn error_value_is_move_only() {
    // ADR-0135 decision 6. Passes today with UseAfterMove.
    let source = with_error_types(
        r#"
func main(): Int {
    val first: Error = new AllocationFailedError();
    val second: Error = first;
    val third: Error = first;
    return 0;
}
"#,
    );
    assert_semantic_rejection("error.move.only", &source);
}
