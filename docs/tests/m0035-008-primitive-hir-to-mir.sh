#!/usr/bin/env bash
set -euo pipefail

grep -q 'M0035-008' docs/tasks/M0035-008-primitive-hir-to-mir.md
rg -q 'HirExpressionKind::UnitLiteral|require_bootstrap_runtime_type' crates/compiler/src/mir.rs
cargo test -p compiler --test mir m0035_hir_to_mir_lowers_primitive_literals_and_unit_return
