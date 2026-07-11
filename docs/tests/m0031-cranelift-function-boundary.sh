#!/usr/bin/env sh
set -eu
rg -q 'cranelift-codegen = "0.133.1"' crates/compiler/Cargo.toml
rg -q 'pub fn lower_mir_function_to_cranelift' crates/compiler/src/backend.rs
rg -q 'TypeArena' crates/compiler/src/backend.rs
rg -q 'm0031_lowers_int_constant_return_to_verified_cranelift_ir' crates/compiler/tests/backend.rs
rg -q 'm0031_rejects_unsupported_mir_instruction' crates/compiler/tests/backend.rs
cargo test -p compiler --test backend
