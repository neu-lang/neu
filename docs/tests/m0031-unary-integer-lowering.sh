#!/usr/bin/env sh
set -eu
rg -q 'MirUnary::Negate' crates/compiler/src/backend.rs
rg -q 'TrapCode::INTEGER_OVERFLOW' crates/compiler/src/backend.rs
rg -q 'm0031_lowers_unary_int_operations' crates/compiler/tests/backend.rs
cargo test -p compiler --test backend m0031_lowers_unary_int_operations
