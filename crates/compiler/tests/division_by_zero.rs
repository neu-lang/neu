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
        std::env::temp_dir().join(format!("neu-division-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(13200),
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

#[test]
fn int_division_by_a_runtime_zero_writes_a_message_and_traps() {
    // The divisor comes from a call, so the zero is not a compile-time constant.
    let source = r#"
        func zero(): Int { return 0; }
        func main(): Int {
            return 10 / zero();
        }
    "#;
    let (code, stderr) = run_capturing(source, "int_divide");
    assert!(is_trap(code), "{stderr}");
    assert!(stderr.contains("neu: division by zero"), "{stderr}");
}

#[test]
fn int_remainder_by_a_runtime_zero_writes_a_message_and_traps() {
    let source = r#"
        func zero(): Int { return 0; }
        func main(): Int {
            return 10 % zero();
        }
    "#;
    let (code, stderr) = run_capturing(source, "int_remainder");
    assert!(is_trap(code), "{stderr}");
    assert!(stderr.contains("neu: division by zero"), "{stderr}");
}

#[test]
fn byte_division_by_a_runtime_zero_writes_a_message_and_traps() {
    let source = r#"
        func zero(): Byte { val z: Byte = 0; return z; }
        func main(): Int {
            val nine: Byte = 9;
            val quotient: Byte = nine / zero();
            return 1;
        }
    "#;
    let (code, stderr) = run_capturing(source, "byte_divide");
    assert!(is_trap(code), "{stderr}");
    assert!(stderr.contains("neu: division by zero"), "{stderr}");
}

#[test]
fn division_and_remainder_by_a_nonzero_runtime_value_are_unchanged() {
    let source = r#"
        func two(): Int { return 2; }
        func main(): Int {
            return 10 / two() + 10 % 3;
        }
    "#;
    let (code, stderr) = run_capturing(source, "nonzero_divide");
    assert_eq!(code, Some(6), "{stderr}");
    assert!(stderr.is_empty(), "{stderr}");
}
