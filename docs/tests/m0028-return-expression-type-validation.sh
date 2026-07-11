#!/usr/bin/env sh
set -eu

rg -q 'check_m0028_return_expression_types' crates/compiler/src/type_check.rs
rg -q 'm0028_return_expression_types_report_known_mismatches_only' crates/compiler/tests/type_check.rs
cargo test -p compiler --test type_check m0028_return_expression_types
printf '%s\n' 'm0028 return expression type validation passed'
