#!/usr/bin/env sh
set -eu

rg -q 'pub enum ThreadCapability' crates/compiler/src/thread.rs
rg -q 'satisfies_thread_capability' crates/compiler/src/thread.rs
rg -q 'pub mod thread;' crates/compiler/src/lib.rs
rg -q 'm0024_primitives_follow_adr0037_capabilities' crates/compiler/tests/thread.rs
rg -q 'm0024_nullable_capabilities_follow_base_type' crates/compiler/tests/thread.rs
rg -q 'm0024_nominal_generic_and_missing_types_satisfy_no_capabilities' crates/compiler/tests/thread.rs

cargo test -p compiler --test thread

printf '%s\n' 'm0024 thread capability derivation contract passed'
