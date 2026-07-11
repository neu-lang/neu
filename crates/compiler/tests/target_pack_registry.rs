use std::path::PathBuf;

use compiler::target_pack::{
    TargetCapabilities, TargetPackManifest, TargetPackRegistry, TargetPackRegistryError,
};
use target_lexicon::Triple;

fn bundled_pack_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target-packs")
}

#[test]
fn resolves_the_explicit_host_target_pack() {
    let registry = TargetPackRegistry::new(bundled_pack_root());
    let pack = registry.resolve(Triple::host()).expect("host target pack");

    assert_eq!(pack.target(), Triple::host());
    assert_eq!(pack.capabilities(), &TargetCapabilities::bootstrap_host());
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

#[test]
fn rejects_a_profile_that_does_not_match_the_bootstrap_contract() {
    let manifest = r#"
[target]
triple = "aarch64-apple-darwin"
object_format = "macho"
executable_format = "macho"

[linker]
path = "bin/ld64.lld"

[startup_shim]
path = "runtime/startup.o"

[entry]
platform_symbol = "_start"
language_symbol = "neu_lang_main"
trap_exit_code = 1

[capabilities]
int_width_bits = 32
pointer_width_bits = 64
endianness = "little"
alignment_model = "deferred"
calling_convention = "platform-default"
atomic_model = "deferred"
platform_apis = []
"#;

    assert_eq!(
        TargetPackManifest::from_toml(manifest),
        Err(compiler::target_pack::TargetPackError::InvalidCapabilities)
    );
}
