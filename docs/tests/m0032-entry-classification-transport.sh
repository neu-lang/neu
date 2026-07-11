#!/usr/bin/env sh
set -eu
rg -q 'entry: bool' crates/compiler/src/mir.rs
rg -q 'm0032_hir_to_mir_preserves_entry_classification' crates/compiler/tests/mir.rs
cargo test -p compiler --test mir m0032_hir_to_mir_preserves_entry_classification
