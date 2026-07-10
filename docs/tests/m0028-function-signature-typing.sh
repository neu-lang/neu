#!/usr/bin/env sh
set -eu
rg -q 'type_m0028_function_signatures' crates/compiler/src/type_check.rs
rg -q 'm0028_function_signatures_type_explicit_int_parameters_and_returns' crates/compiler/tests/type_check.rs
cargo test -p compiler --test type_check m0028_function_signatures
printf '%s\n' 'm0028 function signature typing passed'
