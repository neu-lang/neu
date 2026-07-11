#!/bin/sh
set -eu

grep -q 'M0032-012' docs/tasks/M0032-012-executable-smoke-runner.md
grep -q 'fn m0032_runs_linked_output_without_arguments' crates/compiler/tests/linker.rs
grep -q 'fn m0032_reports_unavailable_executable' crates/compiler/tests/linker.rs
cargo test -p compiler --test linker m0032_runs_linked_output_without_arguments
