#!/usr/bin/env sh
set -eu

rg -q 'apply_m0028_direct_call_results' crates/compiler/src/type_check.rs
rg -q 'm0028_executable_core_accepts_checked_direct_calls' crates/compiler/tests/type_check.rs
rg -q 'm0028_executable_core_keeps_invalid_direct_calls_deferred' crates/compiler/tests/type_check.rs
cargo test -p compiler --test type_check m0028_executable_core_
printf '%s\n' 'm0028 direct call core integration passed'
