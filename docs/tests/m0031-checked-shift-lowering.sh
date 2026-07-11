#!/usr/bin/env sh
set -eu
rg -q 'MirArithmetic::ShiftLeft' crates/compiler/src/backend.rs
rg -q 'MirArithmetic::ShiftRight' crates/compiler/src/backend.rs
rg -q 'INVALID_SHIFT_COUNT_TRAP' crates/compiler/src/backend.rs
rg -q 'm0031_lowers_checked_shifts' crates/compiler/tests/backend.rs
cargo test -p compiler --test backend m0031_lowers_checked_shifts
