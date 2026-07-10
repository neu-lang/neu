#!/usr/bin/env sh
set -eu

rg -q 'build_scoped_binding_index' crates/newlang/src/name_resolution.rs
rg -q 'm0021_combined_binding_index_resolves_parameter_use' crates/newlang/tests/name_resolution.rs
echo 'm0021-combined-parameter-local-binding-index: contract validated'
