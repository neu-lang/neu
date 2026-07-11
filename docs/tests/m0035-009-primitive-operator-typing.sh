#!/usr/bin/env bash
set -euo pipefail

grep -q 'M0035-009' docs/tasks/M0035-009-primitive-operator-typing.md
rg -q 'type_m0035_primitive_operators' crates/compiler/src/type_check.rs
rg -q 'ParsedUnaryOperator::Not' crates/compiler/src/parser.rs
cargo test -p compiler --test type_check m0035_primitive_operators_type_bool_float_and_byte_families
