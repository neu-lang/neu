#!/usr/bin/env bash
set -euo pipefail

grep -q 'M0035-006' docs/tasks/M0035-006-primitive-cranelift-lowering.md
rg -q 'FloatConstant|BoolConstant|ByteConstant|ReturnUnit' crates/compiler/src/backend.rs
cargo test -p compiler --test backend m0035_lowers_bool_byte_float_and_unit_returns
