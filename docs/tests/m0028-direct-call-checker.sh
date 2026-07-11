#!/usr/bin/env sh
set -eu

rg -q 'check_m0028_direct_calls' crates/compiler/src/type_check.rs
rg -q 'm0028_direct_calls_reject_every_edge_in_a_recursive_cycle' crates/compiler/tests/type_check.rs
cargo test -p compiler --test type_check m0028_direct_calls
printf '%s\n' 'm0028 direct call checker passed'
