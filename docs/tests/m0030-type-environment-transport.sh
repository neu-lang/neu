#!/usr/bin/env sh
set -eu
rg -q 'pub fn lower_hir_to_mir' crates/compiler/src/mir.rs
rg -q 'types: &TypeArena' crates/compiler/src/mir.rs
rg -q 'UnsupportedRuntimeType' crates/compiler/src/mir.rs
rg -q 'm0030_hir_to_mir_requires_owning_type_arena' crates/compiler/tests/mir.rs
cargo test -p compiler --test mir m0030_hir_to_mir_requires_owning_type_arena
