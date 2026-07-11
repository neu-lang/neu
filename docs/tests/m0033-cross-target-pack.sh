#!/usr/bin/env bash
set -euo pipefail

grep -q 'M0033-003' docs/tasks/M0033-003-cross-target-object-smoke.md
test -x target-packs/x86_64-unknown-linux-gnu/bin/ld.lld
test -f target-packs/x86_64-unknown-linux-gnu/runtime/startup.o
grep -q 'triple = "x86_64-unknown-linux-gnu"' target-packs/x86_64-unknown-linux-gnu/manifest.toml
rg -q 'emit_mir_function_to_object_for_target' crates/compiler/src/backend.rs
cargo test -p compiler --test cross_target_pack
