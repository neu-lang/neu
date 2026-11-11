use std::{fs, process::Command};

use compiler::{
    driver::{SourceDriverOptions, compile_source_to_executable},
    module::{ModuleName, PackageNamespace},
    source::SourceFileId,
};

fn compile_and_run(source: &str, name: &str) -> i32 {
    let workspace =
        std::env::temp_dir().join(format!("neu-suspend-member-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(17000),
            ModuleName::parse(name).unwrap(),
            PackageNamespace::root(),
            &executable,
        ),
    )
    .unwrap();
    let code = Command::new(&executable)
        .status()
        .unwrap()
        .code()
        .unwrap_or(-1);
    let _ = fs::remove_dir_all(workspace);
    code
}

#[test]
fn suspend_interface_method_compiles_and_dispatches() {
    // Previously the class-body parser looped forever on `suspend` members.
    let source = r#"
        public interface Source { suspend func read(): Int; }
        class Constant: Source { suspend func read(): Int { return 12; } }
        public suspend func main(): Int {
            val source: Source = new Constant();
            return source.read();
        }
    "#;
    assert_eq!(compile_and_run(source, "iface_dispatch"), 12);
}

#[test]
fn visibility_then_suspend_method_parses() {
    let source = r#"
        class Worker {
            public suspend func work(): Int { return 5; }
        }
        public suspend func main(): Int {
            val worker = new Worker();
            return worker.work();
        }
    "#;
    assert_eq!(compile_and_run(source, "visibility_suspend"), 5);
}

#[test]
fn malformed_class_member_terminates_with_a_diagnostic() {
    // The class-body parser must always make progress, so a stray token is a
    // diagnostic, not an infinite loop.
    let output = std::env::temp_dir().join(format!("neu-malformed-member-{}", std::process::id()));
    let error = compile_source_to_executable(
        "class Bad { 42 } public suspend func main(): Int { return 0; }",
        SourceDriverOptions::new(
            SourceFileId::from_raw(17010),
            ModuleName::parse("malformed_member").unwrap(),
            PackageNamespace::root(),
            &output,
        ),
    );
    assert!(error.is_err());
}
