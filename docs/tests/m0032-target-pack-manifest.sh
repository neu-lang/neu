#!/usr/bin/env sh
set -eu
rg -q 'from_toml' crates/compiler/src/target_pack.rs
rg -q 'm0032_loads_target_pack_manifest_from_toml' crates/compiler/tests/target_pack.rs
cargo test -p compiler --test target_pack m0032_loads_target_pack_manifest_from_toml
