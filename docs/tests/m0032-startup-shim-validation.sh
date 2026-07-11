#!/bin/sh
set -eu

grep -q 'M0032-013' docs/tasks/M0032-013-startup-shim-object-validation.md
grep -q 'fn m0032_rejects_malformed_startup_shim_object' crates/compiler/tests/target_pack.rs
grep -q 'fn m0032_rejects_startup_shim_without_platform_entry' crates/compiler/tests/target_pack.rs
grep -q 'fn m0032_accepts_startup_shim_language_entry_relocation' crates/compiler/tests/target_pack.rs
cargo test -p compiler --test target_pack m0032_rejects_malformed_startup_shim_object
