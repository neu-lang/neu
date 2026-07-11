#!/usr/bin/env sh
set -eu
rg -q 'emit_mir_function_to_object_with_entry_symbol' crates/compiler/src/backend.rs
rg -q 'm0032_emits_canonical_language_entry_symbol' crates/compiler/tests/object.rs
cargo test -p compiler --test object m0032_emits_canonical_language_entry_symbol
