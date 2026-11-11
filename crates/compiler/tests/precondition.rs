use std::{fs, process::Command};

use compiler::{
    driver::{SourceDriverOptions, compile_source_to_executable},
    module::{ModuleName, PackageNamespace},
    source::SourceFileId,
};

/// A trap ends the process by signal on Unix. On Windows it is an illegal
/// instruction, which the process reports as this exit code.
const WINDOWS_ILLEGAL_INSTRUCTION: i32 = -1073741795;

fn is_trap(code: Option<i32>) -> bool {
    code.is_none() || code == Some(WINDOWS_ILLEGAL_INSTRUCTION)
}

/// Compiles and runs `source`, returning the exit code (None when a signal ended
/// the process) and everything the program wrote to stderr.
fn run_capturing(source: &str, name: &str) -> (Option<i32>, String) {
    let workspace =
        std::env::temp_dir().join(format!("neu-precondition-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(13300),
            ModuleName::parse(name).unwrap(),
            PackageNamespace::root(),
            &executable,
        ),
    )
    .unwrap();
    let output = Command::new(&executable).output().unwrap();
    let _ = fs::remove_dir_all(workspace);
    (
        output.status.code(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

/// The real `stdlib/core/numeric.neu`, without its test function.
fn numeric_helpers() -> &'static str {
    let source = include_str!("../../../stdlib/core/numeric.neu");
    source
        .split("public test func")
        .next()
        .expect("numeric helpers precede the test function")
}

/// Windows writes `\r\n` for the runtime's newline. The message text is the same.
fn with_unix_newlines(stderr: &str) -> String {
    stderr.replace("\r\n", "\n")
}

#[test]
fn precondition_that_holds_continues() {
    let source = r#"
        func main(): Int {
            precondition(1 < 2, "one is less than two");
            return 3;
        }
    "#;
    let (code, stderr) = run_capturing(source, "holds");
    assert_eq!(code, Some(3), "{stderr}");
    assert!(stderr.is_empty(), "{stderr}");
}

#[test]
fn precondition_that_fails_writes_its_message_and_traps() {
    let source = r#"
        func main(): Int {
            precondition(1 > 2, "one is not greater than two");
            return 4;
        }
    "#;
    let (code, stderr) = run_capturing(source, "fails");
    assert!(is_trap(code), "{stderr}");
    assert_eq!(
        with_unix_newlines(&stderr),
        "neu: precondition failed: one is not greater than two\n"
    );
}

#[test]
fn clamp_with_ordered_bounds_returns_the_bounded_value() {
    let source = format!(
        "{}\n{}",
        numeric_helpers(),
        r#"
        func main(): Int {
            return clamp(9, 0, 5);
        }
        "#
    );
    let (code, stderr) = run_capturing(&source, "clamp_ordered");
    assert_eq!(code, Some(5), "{stderr}");
    assert!(stderr.is_empty(), "{stderr}");
}

#[test]
fn clamp_with_inverted_bounds_traps_with_the_precondition_message() {
    // ADR-0112: `lower <= upper` is a precondition, so an inverted range is a
    // programmer fault that traps, not an arbitrary result.
    let source = format!(
        "{}\n{}",
        numeric_helpers(),
        r#"
        func main(): Int {
            return clamp(3, 5, 0);
        }
        "#
    );
    let (code, stderr) = run_capturing(&source, "clamp_inverted");
    assert!(is_trap(code), "{stderr}");
    assert_eq!(
        with_unix_newlines(&stderr),
        "neu: precondition failed: clamp requires lower <= upper\n"
    );
}

#[test]
fn abs_of_the_smallest_int_traps_as_an_overflow() {
    // ADR-0112 and ADR-0043: abs(Int.MIN) is an arithmetic overflow, not a wrap.
    let source = format!(
        "{}\n{}",
        numeric_helpers(),
        r#"
        func smallest(): Int { return -9223372036854775807 - 1; }
        func main(): Int {
            return abs(smallest());
        }
        "#
    );
    let (code, stderr) = run_capturing(&source, "abs_min");
    assert!(is_trap(code), "{stderr}");
    assert!(!stderr.contains("precondition"), "{stderr}");
}

#[test]
fn numeric_helpers_are_correct_at_the_int_extremes() {
    // Each `if` adds one to `score` when its expectation holds, so the exit code
    // counts the correct results. Five expectations are checked.
    let source = format!(
        "{}\n{}",
        numeric_helpers(),
        r#"
        func smallest(): Int { return -9223372036854775807 - 1; }
        func largest(): Int { return 9223372036854775807; }
        func main(): Int {
            var score = 0;
            val lo = smallest();
            val hi = largest();
            if min(lo, hi) == lo { score = score + 1; }
            if max(lo, hi) == hi { score = score + 1; }
            if clamp(hi, lo, hi) == hi { score = score + 1; }
            if sign(lo) == -1 { score = score + 1; }
            if sign(hi) == 1 { score = score + 1; }
            return score;
        }
        "#
    );
    let (code, stderr) = run_capturing(&source, "extremes");
    assert_eq!(code, Some(5), "{stderr}");
}
