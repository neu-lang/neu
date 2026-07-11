#!/usr/bin/env sh
set -eu
rg -q 'MirArithmetic::Remainder' crates/compiler/src/backend.rs
rg -q 'm0031_lowers_checked_remainder' crates/compiler/tests/backend.rs
cargo test -p compiler --test backend m0031_lowers_checked_remainder
