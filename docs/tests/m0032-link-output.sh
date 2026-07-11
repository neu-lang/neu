#!/bin/sh
set -eu

grep -q 'M0032-011' docs/tasks/M0032-011-link-output-verification.md
grep -q 'fn m0032_accepts_linker_output_file' crates/compiler/tests/linker.rs
grep -q 'fn m0032_rejects_linker_success_without_output' crates/compiler/tests/linker.rs
cargo test -p compiler --test linker m0032_rejects_linker_success_without_output
