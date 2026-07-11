#!/bin/sh
set -eu

grep -q 'M0032-016' docs/tasks/M0032-016-host-target-pack-smoke.md
grep -q 'm0032_links_and_runs_host_target_pack_smoke' crates/compiler/tests/host_target_pack.rs
test -x target-packs/aarch64-apple-darwin/bin/ld64.lld
test -f target-packs/aarch64-apple-darwin/runtime/startup.o
grep -q 'language_symbol = "neu_lang_main"' target-packs/aarch64-apple-darwin/manifest.toml
cargo test -p compiler --test host_target_pack m0032_links_and_runs_host_target_pack_smoke
