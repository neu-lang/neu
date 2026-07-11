#!/bin/sh
set -eu

grep -q 'M0032-015' docs/tasks/M0032-015-bootstrap-outcome-smoke-verification.md
grep -q 'fn m0032_verifies_valid_bootstrap_exit' crates/compiler/tests/linker.rs
grep -q 'fn m0032_verifies_unsupported_bootstrap_exit' crates/compiler/tests/linker.rs
cargo test -p compiler --test linker m0032_verifies_valid_bootstrap_exit
