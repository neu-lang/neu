#!/usr/bin/env sh
set -eu
rg -q 'MirUnary' crates/compiler/src/mir.rs
rg -q 'm0030_hir_unary_ints_lower_to_mir' crates/compiler/tests/mir.rs
cargo test -p compiler --test mir m0030_hir_unary_ints_lower_to_mir
