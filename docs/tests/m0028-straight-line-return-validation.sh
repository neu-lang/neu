#!/usr/bin/env sh
set -eu

rg -q 'ADR-0050: Bootstrap Straight-Line Return Diagnostics' docs/SPEC.md
rg -q 'check_m0028_straight_line_returns' crates/compiler/src/type_check.rs
rg -q 'm0028_straight_line_return_validation_reports_missing_and_unreachable_returns' crates/compiler/tests/type_check.rs
cargo test -p compiler --test type_check m0028_straight_line_return
printf '%s\n' 'm0028 straight-line return validation passed'
