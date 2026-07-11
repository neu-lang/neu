#!/usr/bin/env bash
set -euo pipefail

grep -q 'M0035-012' docs/tasks/M0035-012-primitive-cranelift-operations.md
rg -q 'lower_basic_arithmetic|MirInstruction::Compare|LogicalNot' crates/compiler/src/backend.rs
cargo test -p compiler --test backend m0035_lowers_float_byte_bool_and_comparison_operations
