use std::{fs, path::PathBuf};

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
