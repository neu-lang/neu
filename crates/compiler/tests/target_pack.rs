use std::{fs, path::PathBuf};

use compiler::target_pack::{ArtifactKind, TargetPack, TargetPackError, TargetPackManifest};
use target_lexicon::Triple;

fn fixture_root(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("neu-target-pack-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("bin")).unwrap();
    fs::create_dir_all(root.join("runtime")).unwrap();
    root
}

fn manifest(linker: &str, startup_shim: &str) -> TargetPackManifest {
    TargetPackManifest::new(
        Triple::host(),
        "macho",
        "macho",
        linker,
        startup_shim,
        "_start",
        "neu_lang_main",
        1,
    )
    .unwrap()
}

#[test]
fn m0032_resolves_valid_target_pack() {
    let root = fixture_root("valid");
    let linker = root.join("bin/linker");
    let shim = root.join("runtime/startup.o");
    fs::write(&linker, b"linker").unwrap();
    fs::write(&shim, b"shim").unwrap();

    let pack = TargetPack::resolve(
        &root,
        manifest("bin/linker", "runtime/startup.o"),
        Triple::host(),
    )
    .unwrap();

    assert_eq!(pack.target(), Triple::host());
    assert_eq!(pack.linker_path(), linker.canonicalize().unwrap());
    assert_eq!(pack.startup_shim_path(), shim.canonicalize().unwrap());
    assert_eq!(pack.entry_symbol(), "_start");
    assert_eq!(pack.language_entry_symbol(), "neu_lang_main");
    let _ = fs::remove_dir_all(root);
}

#[test]
fn m0032_rejects_target_mismatch() {
    let root = fixture_root("target-mismatch");
    fs::write(root.join("bin/linker"), b"linker").unwrap();
    fs::write(root.join("runtime/startup.o"), b"shim").unwrap();
    let other_target = "x86_64-unknown-linux-gnu".parse::<Triple>().unwrap();

    assert_eq!(
        TargetPack::resolve(
            &root,
            manifest("bin/linker", "runtime/startup.o"),
            other_target
        ),
        Err(TargetPackError::TargetMismatch)
    );
    let _ = fs::remove_dir_all(root);
}

#[test]
fn m0032_rejects_unsafe_and_missing_artifacts() {
    let root = fixture_root("unsafe");
    fs::write(root.join("bin/linker"), b"linker").unwrap();
    fs::write(root.join("runtime/startup.o"), b"shim").unwrap();

    assert_eq!(
        TargetPack::resolve(
            &root,
            manifest("../outside", "runtime/startup.o"),
            Triple::host()
        ),
        Err(TargetPackError::TraversalArtifactPath(ArtifactKind::Linker))
    );
    assert_eq!(
        TargetPack::resolve(
            &root,
            manifest("/tmp/host-linker", "runtime/startup.o"),
            Triple::host(),
        ),
        Err(TargetPackError::AbsoluteArtifactPath(ArtifactKind::Linker))
    );
    assert_eq!(
        TargetPack::resolve(
            &root,
            manifest("bin/missing", "runtime/startup.o"),
            Triple::host()
        ),
        Err(TargetPackError::MissingArtifact(ArtifactKind::Linker))
    );
    let _ = fs::remove_dir_all(root);
}

#[test]
fn m0032_rejects_invalid_manifest() {
    assert_eq!(
        TargetPackManifest::new(
            Triple::host(),
            "",
            "macho",
            "bin/linker",
            "runtime/startup.o",
            "_start",
            "neu_lang_main",
            1,
        ),
        Err(TargetPackError::InvalidManifest)
    );
    assert_eq!(
        TargetPackManifest::new(
            Triple::host(),
            "macho",
            "macho",
            "bin/linker",
            "runtime/startup.o",
            "_start",
            "neu_lang_main",
            0,
        ),
        Err(TargetPackError::InvalidManifest)
    );
}
