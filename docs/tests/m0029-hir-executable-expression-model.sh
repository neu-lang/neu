#!/usr/bin/env sh
set -eu

rg -q 'pub enum HirBinaryOperator' crates/compiler/src/hir.rs
rg -q 'm0029_hir_executable_expressions_preserve_ordered_operands_and_assignments' crates/compiler/tests/hir.rs
cargo test -p compiler --test hir m0029_hir_executable_expressions
printf '%s\n' 'm0029 HIR executable expression model passed'
