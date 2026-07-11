#!/usr/bin/env sh
set -eu
rg -q 'return_type: TypeId' crates/compiler/src/mir.rs
rg -q 'pub fn return_type\(&self\) -> TypeId' crates/compiler/src/mir.rs
rg -q 'm0030_mir_function_preserves_declared_return_type' crates/compiler/tests/mir.rs
cargo test -p compiler --test mir m0030_mir_function_preserves_declared_return_type
