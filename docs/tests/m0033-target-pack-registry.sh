#!/usr/bin/env bash
set -euo pipefail

grep -q 'M0033-001' docs/tasks/M0033-001-target-pack-registry.md
rg -q 'pub struct TargetPackRegistry' crates/compiler/src/target_pack.rs
rg -q 'UnknownTarget' crates/compiler/src/target_pack.rs
! rg -n 'which |command -v|PATH' crates/compiler/src/target_pack.rs
cargo test -p compiler --test target_pack_registry
