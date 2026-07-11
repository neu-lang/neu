#!/bin/sh
set -eu

grep -q 'M0032-010' docs/tasks/M0032-010-link-invocation-execution.md
grep -q 'fn m0032_executes_resolved_linker_successfully' crates/compiler/tests/linker.rs
grep -q 'fn m0032_reports_linker_non_success' crates/compiler/tests/linker.rs
grep -q 'fn m0032_reports_linker_launch_failure' crates/compiler/tests/linker.rs
cargo test -p compiler --test linker m0032_executes_resolved_linker_successfully
