#!/usr/bin/env bash
set -euo pipefail

grep -q 'M0035-007' docs/tasks/M0035-007-primitive-checked-hir-lowering.md
rg -q 'float_literals' crates/compiler/src/parser.rs
rg -q 'ParsedLiteralKind::Float|ParsedLiteralKind::Unit' crates/compiler/src/hir.rs
cargo test -p compiler --test hir m0035_checked_source_lowers_bool_unit_and_float_literals_to_hir
