use std::path::PathBuf;

use compiler::target_pack::{TargetPackRegistry, TargetPackRegistryError};
use target_lexicon::Triple;

fn bundled_pack_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target-packs")
}

#[test]
fn resolves_the_explicit_host_target_pack() {
    let registry = TargetPackRegistry::new(bundled_pack_root());
    let pack = registry.resolve(Triple::host()).expect("host target pack");

    assert_eq!(pack.target(), Triple::host());
}

#[test]
fn rejects_an_unavailable_target_without_host_fallback() {
    let requested = "aarch64-unknown-linux-gnu"
        .parse::<Triple>()
        .expect("valid target triple");
    let registry = TargetPackRegistry::new(bundled_pack_root());

    assert_eq!(
        registry.resolve(requested.clone()),
        Err(TargetPackRegistryError::UnknownTarget(requested))
    );
}
