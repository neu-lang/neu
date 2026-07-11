#!/usr/bin/env sh
set -eu
rg -q 'pub struct TargetPackManifest' crates/compiler/src/target_pack.rs
rg -q 'TargetPackError::TraversalArtifactPath' crates/compiler/src/target_pack.rs
rg -q 'm0032_resolves_valid_target_pack' crates/compiler/tests/target_pack.rs
rg -q 'm0032_rejects_invalid_manifest' crates/compiler/tests/target_pack.rs
cargo test -p compiler --test target_pack
