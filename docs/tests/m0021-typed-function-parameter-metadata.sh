#!/usr/bin/env sh
set -eu

rg -q 'struct ParsedFunctionParameter' crates/compiler/src/parser.rs
rg -q 'function_parameters' crates/compiler/src/parser.rs
rg -q 'm0021_typed_function_parameter_records_function_and_named_type' crates/compiler/tests/parser.rs
echo 'm0021-typed-function-parameter-metadata: contract validated'
