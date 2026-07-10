#!/usr/bin/env sh
set -eu

rg -q 'build_function_parameter_binding_index' crates/compiler/src/name_resolution.rs
rg -q 'm0021_function_parameter_binding_uses_owning_body_scope' crates/compiler/tests/name_resolution.rs
echo 'm0021-function-parameter-binding-index: contract validated'
