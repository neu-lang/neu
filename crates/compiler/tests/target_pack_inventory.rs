use std::{fs, path::PathBuf};

use compiler::target_pack::{TargetPackRegistry, TargetPackRegistryError};
use target_lexicon::Triple;

fn bundled_pack_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target-packs")
}

#[test]
fn lists_all_bundled_targets_in_stable_order() {
    let registry = TargetPackRegistry::new(bundled_pack_root());

    assert_eq!(
        registry.available_targets().unwrap(),
        vec![
            "aarch64-apple-darwin".parse::<Triple>().unwrap(),
            "x86_64-unknown-linux-gnu".parse::<Triple>().unwrap(),
        ]
    );
}

#[test]
fn rejects_a_directory_whose_manifest_names_another_target() {
    let root = std::env::temp_dir().join(format!("neu-pack-inventory-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("x86_64-unknown-linux-gnu")).unwrap();
    fs::write(
        root.join("x86_64-unknown-linux-gnu/manifest.toml"),
        "[target]\ntriple = \"aarch64-apple-darwin\"\nobject_format = \"macho\"\nexecutable_format = \"macho\"\n[linker]\npath = \"bin/linker\"\n[startup_shim]\npath = \"runtime/startup.o\"\n[entry]\nplatform_symbol = \"_start\"\nlanguage_symbol = \"neu_lang_main\"\ntrap_exit_code = 1\n[capabilities]\nint_width_bits = 64\npointer_width_bits = 64\nendianness = \"little\"\nalignment_model = \"deferred\"\ncalling_convention = \"platform-default\"\natomic_model = \"deferred\"\nplatform_apis = []\n",
    )
    .unwrap();

    let registry = TargetPackRegistry::new(&root);
    assert!(matches!(
        registry.available_targets(),
        Err(TargetPackRegistryError::DirectoryTargetMismatch { .. })
    ));
    let _ = fs::remove_dir_all(root);
}
