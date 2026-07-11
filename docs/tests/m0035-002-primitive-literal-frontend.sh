#!/usr/bin/env bash
set -euo pipefail

grep -q 'M0035-002' docs/tasks/M0035-002-primitive-literal-frontend.md
rg -q 'FloatDecimal' crates/compiler/src/lexer.rs crates/compiler/src/parser.rs
rg -q 'ParsedLiteralKind::Unit' crates/compiler/src/parser.rs
cargo test -p compiler --test lexer lexes_decimal_and_exponent_float_literals
cargo test -p compiler --test parser records_float_and_unit_literal_metadata
