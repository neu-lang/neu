#!/usr/bin/env bash
set -euo pipefail

grep -q 'M0035-004' docs/tasks/M0035-004-primitive-hir-model.md
rg -q 'BoolLiteral|UnitLiteral|FloatLiteral|ByteLiteral' crates/compiler/src/hir.rs
cargo test -p compiler --test hir m0035_hir_preserves_non_integer_primitive_literal_payloads
