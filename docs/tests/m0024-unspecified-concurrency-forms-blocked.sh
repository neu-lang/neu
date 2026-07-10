#!/usr/bin/env sh
set -eu

rg -q 'MalformedCoroutineConstruct' crates/compiler/src/parser.rs
rg -q 'm0024_unspecified_concurrency_forms_remain_blocked' crates/compiler/tests/parser.rs
rg -q 'M0024 must not add parser support for concurrency' docs/adr/ADR-0037-bootstrap-thread-capability-analysis.md

if rg -q 'parse_(thread|spawn|task|async|coroutine)' crates/compiler/src/parser.rs; then
  echo "m0024 unspecified concurrency forms: parser contains source-level concurrency parser API" >&2
  exit 1
fi

cargo test -p compiler --test parser m0024_unspecified_concurrency_forms_remain_blocked

printf '%s\n' 'm0024 unspecified concurrency forms blocked contract passed'
