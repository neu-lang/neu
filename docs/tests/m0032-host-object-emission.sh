#!/usr/bin/env sh
set -eu
rg -q 'emit_mir_function_to_object' crates/compiler/src/backend.rs
rg -q 'm0032_emits_host_object_for_int_return' crates/compiler/tests/object.rs
cargo test -p compiler --test object m0032_emits_host_object_for_int_return
