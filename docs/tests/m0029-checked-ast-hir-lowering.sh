#!/usr/bin/env sh
set -eu

rg -q 'lower_checked_hir_source' crates/compiler/src/hir.rs
rg -q 'm0029_checked_source_lowers_integer_helpers_and_direct_calls' crates/compiler/tests/hir.rs
cargo test -p compiler --test hir m0029_checked_source_lowers
printf '%s\n' 'm0029 checked AST HIR lowering passed'
