#!/usr/bin/env sh
set -eu
rg -q 'pub struct LinkInvocation' crates/compiler/src/linker.rs
rg -q 'm0032_builds_deterministic_link_invocation_plan' crates/compiler/tests/linker.rs
cargo test -p compiler --test linker
