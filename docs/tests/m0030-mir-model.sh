#!/usr/bin/env sh
set -eu

rg -q 'pub struct MirModule' crates/compiler/src/mir.rs
rg -q 'm0030_mir_model_preserves_ordered_source_mapped_runtime_facts' crates/compiler/tests/mir.rs
cargo test -p compiler --test mir
printf '%s\n' 'm0030 MIR model passed'
