#!/usr/bin/env sh
set -eu
rg -q 'type_m0028_function_signatures_in' crates/compiler/src/type_check.rs
rg -q 'm0028_function_signatures_share_the_caller_owned_module_arena' crates/compiler/tests/type_check.rs
cargo test -p compiler --test type_check m0028_function_signatures
printf '%s\n' 'm0028 module type identity passed'
