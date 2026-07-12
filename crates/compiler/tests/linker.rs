use std::{fs, path::Path, process::Command};

use compiler::linker::{LinkInvocationError, SystemLinkInvocation};

#[test]
fn host_link_plan_uses_cc_and_preserves_object_order() {
    let root = std::env::temp_dir().join(format!("neu-system-link-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let object = root.join("program.o");
    let output = root.join("program");
    fs::write(&object, b"object").unwrap();

    let plan = SystemLinkInvocation::new(&object, &output).unwrap();
    assert_eq!(plan.program(), Path::new("cc"));
    assert_eq!(plan.arguments()[0], "-o");
    assert_eq!(plan.arguments()[1], output.as_os_str());
    assert_eq!(plan.arguments()[2], object.as_os_str());
    assert_eq!(plan.entry_symbol(), "main");
    let _ = fs::remove_dir_all(root);
}

#[test]
fn neu_linker_overrides_cc() {
    let root = std::env::temp_dir().join(format!("neu-linker-override-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let object = root.join("program.o");
    fs::write(&object, b"object").unwrap();
    let old = std::env::var_os("NEU_LINKER");
    unsafe {
        std::env::set_var("NEU_LINKER", "/bin/echo");
    }
    let plan = SystemLinkInvocation::new(&object, root.join("program")).unwrap();
    assert_eq!(plan.program(), Path::new("/bin/echo"));
    match old {
        Some(value) => unsafe { std::env::set_var("NEU_LINKER", value) },
        None => unsafe { std::env::remove_var("NEU_LINKER") },
    }
    let _ = fs::remove_dir_all(root);
}

#[test]
fn host_link_plan_rejects_missing_object() {
    let root = std::env::temp_dir().join(format!("neu-link-missing-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    assert_eq!(
        SystemLinkInvocation::new(root.join("missing.o"), root.join("program")),
        Err(LinkInvocationError::MissingObject)
    );
    let _ = fs::remove_dir_all(root);
}

#[test]
fn host_linker_can_execute_a_compiled_program() {
    let root = std::env::temp_dir().join(format!("neu-link-execute-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let source = root.join("main.c");
    let object = root.join("main.o");
    let output = root.join("program");
    fs::write(&source, b"int main(void) { return 7; }\n").unwrap();
    assert!(
        Command::new("cc")
            .args([
                "-c",
                source.to_str().unwrap(),
                "-o",
                object.to_str().unwrap()
            ])
            .status()
            .unwrap()
            .success()
    );
    let plan = SystemLinkInvocation::new(&object, &output).unwrap();
    plan.execute().unwrap();
    assert_eq!(
        plan.run().unwrap(),
        compiler::linker::ExecutableRunOutcome::Exited(7)
    );
    let _ = fs::remove_dir_all(root);
}
