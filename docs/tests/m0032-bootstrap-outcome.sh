#!/usr/bin/env sh
set -eu
rg -q 'pub enum BootstrapOutcome' crates/compiler/src/bootstrap.rs
rg -q 'm0032_maps_bootstrap_exit_boundaries' crates/compiler/tests/bootstrap.rs
cargo test -p compiler --test bootstrap
