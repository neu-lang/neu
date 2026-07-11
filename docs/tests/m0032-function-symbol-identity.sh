#!/usr/bin/env sh
set -eu
rg -q 'FunctionSymbolIdentity' crates/compiler/src/module.rs
rg -q 'symbol_identity' crates/compiler/src/hir.rs
rg -q 'symbol_identity' crates/compiler/src/mir.rs
rg -q 'm0032_checked_source_preserves_function_symbol_identity' crates/compiler/tests/hir.rs
rg -q 'm0032_hir_to_mir_preserves_function_symbol_identity' crates/compiler/tests/mir.rs
cargo test -p compiler --test hir m0032_checked_source_preserves_function_symbol_identity
cargo test -p compiler --test mir m0032_hir_to_mir_preserves_function_symbol_identity
