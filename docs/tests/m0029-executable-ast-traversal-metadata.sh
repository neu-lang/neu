#!/usr/bin/env sh
set -eu

rg -q 'ParsedExecutableBodyStatement' crates/compiler/src/parser.rs
rg -q 'm0029_records_executable_body_statements_in_function_source_order' crates/compiler/tests/parser.rs
cargo test -p compiler --test parser m0029_records_executable_body_statements
printf '%s\n' 'm0029 executable AST traversal metadata passed'
