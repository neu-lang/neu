#!/usr/bin/env bash
set -euo pipefail

grep -q 'M0035-005' docs/tasks/M0035-005-primitive-mir-model.md
rg -q 'BoolConstant|FloatConstant|ByteConstant|UnitConstant|ReturnUnit' crates/compiler/src/mir.rs
cargo test -p compiler --test mir m0035_mir_preserves_non_integer_constants_and_unit_return
