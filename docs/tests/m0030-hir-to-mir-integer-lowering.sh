#!/usr/bin/env sh
set -eu
rg -q 'lower_hir_to_mir' crates/compiler/src/mir.rs
rg -q 'm0030_hir_integer_function_lowers_to_ordered_mir_block' crates/compiler/tests/mir.rs
cargo test -p compiler --test mir m0030_hir_integer_function_lowers
