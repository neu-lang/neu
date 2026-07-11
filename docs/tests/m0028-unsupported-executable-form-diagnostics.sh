#!/usr/bin/env sh
set -eu

rg -q 'check_m0028_unsupported_executable_forms' crates/compiler/src/type_check.rs
rg -q 'm0028_unsupported_executable_forms_report_outermost_source_spans' crates/compiler/tests/type_check.rs
cargo test -p compiler --test type_check m0028_unsupported_executable_forms
printf '%s\n' 'm0028 unsupported executable-form diagnostics passed'
