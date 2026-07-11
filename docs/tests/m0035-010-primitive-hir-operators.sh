#!/usr/bin/env bash
set -euo pipefail

grep -q 'M0035-010' docs/tasks/M0035-010-primitive-hir-operators.md
rg -q 'LogicalAnd|LogicalOr|LessEqual' crates/compiler/src/hir.rs
rg -q 'lower_unary_operator' crates/compiler/src/hir.rs
cargo test -p compiler --test hir m0035_hir_preserves_primitive_operator_kinds_and_operand_order
