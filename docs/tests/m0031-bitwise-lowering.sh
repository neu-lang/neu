#!/usr/bin/env sh
set -eu
rg -q 'MirArithmetic::BitwiseAnd' crates/compiler/src/backend.rs
rg -q 'MirArithmetic::BitwiseOr' crates/compiler/src/backend.rs
rg -q 'MirArithmetic::BitwiseXor' crates/compiler/src/backend.rs
rg -q 'm0031_lowers_bitwise_operations' crates/compiler/tests/backend.rs
cargo test -p compiler --test backend m0031_lowers_bitwise_operations
