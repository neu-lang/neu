#!/usr/bin/env sh
set -eu
rg -q 'MirArithmetic::Multiply' crates/compiler/src/backend.rs
rg -q 'smulhi' crates/compiler/src/backend.rs
rg -q 'm0031_lowers_checked_multiplication_with_overflow_trap' crates/compiler/tests/backend.rs
cargo test -p compiler --test backend m0031_lowers_checked_multiplication_with_overflow_trap
