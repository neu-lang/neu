use std::{fs, process::Command};

use compiler::{
    driver::{SourceDriverOptions, compile_source_to_executable},
    module::{ModuleName, PackageNamespace},
    source::SourceFileId,
};

fn compile_and_run(source: &str, name: &str) -> i32 {
    let workspace =
        std::env::temp_dir().join(format!("neu-bool-operators-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(13100),
            ModuleName::parse(name).unwrap(),
            PackageNamespace::root(),
            &executable,
        ),
    )
    .unwrap();
    let status = Command::new(&executable).status().unwrap();
    let code = status.code().unwrap_or(-1);
    let _ = fs::remove_dir_all(workspace);
    code
}

#[test]
fn logical_and_on_literals_is_evaluated() {
    let source = r#"
        func main(): Int {
            if true && false { return 1; }
            return 2;
        }
    "#;
    assert_eq!(compile_and_run(source, "and_literals"), 2);
}

#[test]
fn logical_or_on_literals_is_evaluated() {
    let source = r#"
        func main(): Int {
            if false || true { return 3; }
            return 4;
        }
    "#;
    assert_eq!(compile_and_run(source, "or_literals"), 3);
}

#[test]
fn logical_and_on_bool_locals_is_evaluated() {
    let source = r#"
        func main(): Int {
            val a = true;
            val b = false;
            if a && !b { return 5; }
            return 6;
        }
    "#;
    assert_eq!(compile_and_run(source, "and_locals"), 5);
}

#[test]
fn logical_and_skips_its_right_operand_when_the_left_is_false() {
    // The right operand divides by zero when it runs, so the program traps
    // (a negative exit status on Unix) if `&&` evaluates it.
    let source = r#"
        func boom(d: Int): Bool { return 10 / d == 1; }
        func main(): Int {
            if false && boom(0) { return 1; }
            return 2;
        }
    "#;
    assert_eq!(compile_and_run(source, "and_short_circuit"), 2);
}

#[test]
fn logical_or_skips_its_right_operand_when_the_left_is_true() {
    let source = r#"
        func boom(d: Int): Bool { return 10 / d == 1; }
        func main(): Int {
            if true || boom(0) { return 7; }
            return 8;
        }
    "#;
    assert_eq!(compile_and_run(source, "or_short_circuit"), 7);
}

#[test]
fn logical_operators_in_a_function_without_a_conditional_are_evaluated() {
    // `either` has no `if`, so it takes the lowering path for functions without
    // control flow.
    let source = r#"
        func either(): Bool { return false || true; }
        func main(): Int {
            if either() { return 10; }
            return 11;
        }
    "#;
    assert_eq!(compile_and_run(source, "or_no_conditional"), 10);
}

#[test]
fn logical_and_binds_tighter_than_logical_or() {
    // ADR-0024: `&&` binds tighter than `||`, so this is `true || (false && false)`,
    // which is true.
    let source = r#"
        func main(): Int {
            if true || false && false { return 12; }
            return 13;
        }
    "#;
    assert_eq!(compile_and_run(source, "and_or_precedence"), 12);
}
