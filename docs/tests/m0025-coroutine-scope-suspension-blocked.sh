#!/usr/bin/env sh
set -eu

fail() {
  echo "m0025-coroutine-scope-suspension-blocked: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope implementation path exists: $1"
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

ambiguity=docs/ambiguities/M0025-coroutine-scope-suspension-semantics.md
task=docs/tasks/M0025-001-coroutine-scope-suspension-semantics-blocker.md

require_file "$ambiguity"
require_file "$task"

require_text "$ambiguity" 'Status: `resolved`'
require_text "$ambiguity" 'Required Owner: `main-task semantic design`'
require_text "$ambiguity" 'Which coroutine, async, suspension, and task-scope source forms are approved'
require_text "$ambiguity" 'metadata-only bootstrap subset'
require_text "$ambiguity" 'No implementation may proceed on M0025'
require_text "$ambiguity" 'Accepted `docs/adr/ADR-0038-bootstrap-coroutine-scope-and-suspension-analysis.md`'
require_text "$task" 'Status: `complete`'
require_text "$task" 'An accepted ADR or spec revision must define either a source-syntax subset or a'
require_text "$task" 'No compiler implementation is added'

require_absent_path crates/compiler/src/coroutine.rs
require_absent_path crates/compiler/src/suspension.rs

echo "m0025 coroutine scope and suspension semantics blocker resolution validation passed"
