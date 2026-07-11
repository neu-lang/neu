#!/usr/bin/env sh
set -eu

rg -q 'pub struct HirModule' crates/compiler/src/hir.rs
rg -q 'm0029_hir_model_preserves_typed_source_mapped_executable_facts' crates/compiler/tests/hir.rs
cargo test -p compiler --test hir
printf '%s\n' 'm0029 HIR model passed'
