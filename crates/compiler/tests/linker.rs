use std::{
    fs,
    path::{Path, PathBuf},
};

use compiler::{linker::LinkInvocation, target_pack::TargetPackManifest};
use target_lexicon::Triple;

fn fixture_root(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("neu-link-plan-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("bin")).unwrap();
    fs::create_dir_all(root.join("runtime")).unwrap();
    fs::write(root.join("bin/lld"), b"lld").unwrap();
    fs::write(root.join("runtime/startup.o"), b"shim").unwrap();
    fs::write(root.join("program.o"), b"object").unwrap();
    root
}

#[cfg(unix)]
fn executable_linker(root: &Path, exit_code: i32) {
    use std::os::unix::fs::PermissionsExt;

    fs::write(
        root.join("bin/lld"),
        format!("#!/bin/sh\nexit {exit_code}\n"),
    )
    .unwrap();
    let mut permissions = fs::metadata(root.join("bin/lld")).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(root.join("bin/lld"), permissions).unwrap();
}

#[cfg(unix)]
fn output_producing_linker(root: &Path) {
    use std::os::unix::fs::PermissionsExt;

    fs::write(root.join("bin/lld"), "#!/bin/sh\ntouch \"$2\"\nexit 0\n").unwrap();
    let mut permissions = fs::metadata(root.join("bin/lld")).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(root.join("bin/lld"), permissions).unwrap();
}

#[test]
fn m0032_builds_deterministic_link_invocation_plan() {
    let root = fixture_root("valid");
    let pack = compiler::target_pack::TargetPack::resolve(
        &root,
        TargetPackManifest::new(
            Triple::host(),
            "macho",
            "macho",
            "bin/lld",
            "runtime/startup.o",
            "_start",
            "neu_lang_main",
            1,
        )
        .unwrap(),
        Triple::host(),
    )
    .unwrap();

    let plan = LinkInvocation::new(&pack, root.join("program.o"), root.join("program")).unwrap();
    let canonical_root = root.canonicalize().unwrap();

    assert_eq!(plan.program(), pack.linker_path());
    assert_eq!(plan.arguments()[0], "-o");
    assert_eq!(plan.arguments()[2], "-e");
    assert_eq!(plan.arguments()[3], "_start");
    assert_eq!(
        PathBuf::from(&plan.arguments()[4]),
        canonical_root.join("runtime/startup.o")
    );
    assert_eq!(PathBuf::from(&plan.arguments()[5]), root.join("program.o"));
    assert_eq!(plan.language_entry_symbol(), "neu_lang_main");
    let _ = fs::remove_dir_all(root);
}

#[test]
fn m0032_link_plan_rejects_missing_object() {
    let root = fixture_root("missing-object");
    let pack = compiler::target_pack::TargetPack::resolve(
        &root,
        TargetPackManifest::new(
            Triple::host(),
            "macho",
            "macho",
            "bin/lld",
            "runtime/startup.o",
            "_start",
            "neu_lang_main",
            1,
        )
        .unwrap(),
        Triple::host(),
    )
    .unwrap();

    assert_eq!(
        LinkInvocation::new(&pack, root.join("missing.o"), root.join("program")),
        Err(compiler::linker::LinkInvocationError::MissingObject)
    );
    let _ = fs::remove_dir_all(root);
}

#[cfg(unix)]
#[test]
fn m0032_executes_resolved_linker_successfully() {
    let root = fixture_root("execute-success");
    output_producing_linker(&root);
    let pack = compiler::target_pack::TargetPack::resolve(
        &root,
        TargetPackManifest::new(
            Triple::host(),
            "macho",
            "macho",
            "bin/lld",
            "runtime/startup.o",
            "_start",
            "neu_lang_main",
            1,
        )
        .unwrap(),
        Triple::host(),
    )
    .unwrap();
    let plan = LinkInvocation::new(&pack, root.join("program.o"), root.join("program")).unwrap();

    assert_eq!(plan.execute(), Ok(()));
    let _ = fs::remove_dir_all(root);
}

#[cfg(unix)]
#[test]
fn m0032_reports_linker_non_success() {
    let root = fixture_root("execute-failure");
    executable_linker(&root, 23);
    let pack = compiler::target_pack::TargetPack::resolve(
        &root,
        TargetPackManifest::new(
            Triple::host(),
            "macho",
            "macho",
            "bin/lld",
            "runtime/startup.o",
            "_start",
            "neu_lang_main",
            1,
        )
        .unwrap(),
        Triple::host(),
    )
    .unwrap();
    let plan = LinkInvocation::new(&pack, root.join("program.o"), root.join("program")).unwrap();

    assert_eq!(
        plan.execute(),
        Err(compiler::linker::LinkInvocationError::LinkerFailed(Some(
            23
        )))
    );
    let _ = fs::remove_dir_all(root);
}

#[cfg(unix)]
#[test]
fn m0032_reports_linker_launch_failure() {
    let root = fixture_root("execute-unavailable");
    let pack = compiler::target_pack::TargetPack::resolve(
        &root,
        TargetPackManifest::new(
            Triple::host(),
            "macho",
            "macho",
            "bin/lld",
            "runtime/startup.o",
            "_start",
            "neu_lang_main",
            1,
        )
        .unwrap(),
        Triple::host(),
    )
    .unwrap();
    let plan = LinkInvocation::new(&pack, root.join("program.o"), root.join("program")).unwrap();
    fs::remove_file(root.join("bin/lld")).unwrap();

    assert_eq!(
        plan.execute(),
        Err(compiler::linker::LinkInvocationError::LinkerUnavailable)
    );
    let _ = fs::remove_dir_all(root);
}

#[cfg(unix)]
#[test]
fn m0032_accepts_linker_output_file() {
    let root = fixture_root("output-present");
    output_producing_linker(&root);
    let pack = compiler::target_pack::TargetPack::resolve(
        &root,
        TargetPackManifest::new(
            Triple::host(),
            "macho",
            "macho",
            "bin/lld",
            "runtime/startup.o",
            "_start",
            "neu_lang_main",
            1,
        )
        .unwrap(),
        Triple::host(),
    )
    .unwrap();
    let output = root.join("program");
    let plan = LinkInvocation::new(&pack, root.join("program.o"), &output).unwrap();

    assert_eq!(plan.execute(), Ok(()));
    assert!(output.is_file());
    let _ = fs::remove_dir_all(root);
}

#[cfg(unix)]
#[test]
fn m0032_rejects_linker_success_without_output() {
    let root = fixture_root("output-missing");
    executable_linker(&root, 0);
    let pack = compiler::target_pack::TargetPack::resolve(
        &root,
        TargetPackManifest::new(
            Triple::host(),
            "macho",
            "macho",
            "bin/lld",
            "runtime/startup.o",
            "_start",
            "neu_lang_main",
            1,
        )
        .unwrap(),
        Triple::host(),
    )
    .unwrap();
    let plan = LinkInvocation::new(&pack, root.join("program.o"), root.join("program")).unwrap();

    assert_eq!(
        plan.execute(),
        Err(compiler::linker::LinkInvocationError::MissingOutput)
    );
    let _ = fs::remove_dir_all(root);
}
