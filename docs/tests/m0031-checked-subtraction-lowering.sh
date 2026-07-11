#!/usr/bin/env sh
set -eu
rg -q 'MirArithmetic::Subtract' crates/compiler/src/backend.rs
rg -q 'TrapCode::INTEGER_OVERFLOW' crates/compiler/src/backend.rs
rg -q 'm0031_lowers_checked_subtraction_with_overflow_trap' crates/compiler/tests/backend.rs
cargo test -p compiler --test backend m0031_lowers_checked_subtraction_with_overflow_trap
