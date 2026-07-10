#!/usr/bin/env sh
set -eu

rg -q 'ParsedIntegerLiteral' crates/compiler/src/parser.rs
rg -q 'integer_literals' crates/compiler/src/parser.rs
rg -q 'm0028_records_integer_literal_values_without_truncation' crates/compiler/tests/parser.rs
cargo test -p compiler --test parser

printf '%s\n' 'm0028 integer literal metadata validation passed'
