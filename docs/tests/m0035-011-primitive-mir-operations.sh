#!/usr/bin/env bash
set -euo pipefail

grep -q 'M0035-011' docs/tasks/M0035-011-primitive-mir-operations.md
rg -q 'MirComparison|LogicalNot|Compare' crates/compiler/src/mir.rs
cargo test -p compiler --test mir m0035_hir_to_mir_preserves_boolean_not_and_comparison_operations
cargo test -p compiler --test mir m0035_hir_to_mir_rejects_short_circuit_without_cfg_lowering
