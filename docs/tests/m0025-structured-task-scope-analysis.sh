#!/usr/bin/env sh
set -eu

fail() {
  echo "m0025-structured-task-scope-analysis: $*" >&2
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

task=docs/tasks/M0025-003-structured-task-scope-analysis.md

require_file "$task"
require_file crates/compiler/src/coroutine.rs
require_file crates/compiler/tests/coroutine.rs

require_text "$task" 'Milestone: `M0025`'
require_text "$task" 'Status: `complete`'
require_text crates/compiler/src/lib.rs 'pub mod coroutine;'
require_text crates/compiler/src/coroutine.rs 'TaskScopeEscape'
require_text crates/compiler/src/coroutine.rs 'analyze_structured_task_scopes'
require_text crates/compiler/tests/coroutine.rs 'm0025_structured_scope_accepts_completed_children'
require_text crates/compiler/tests/coroutine.rs 'm0025_structured_scope_reports_child_task_escape'
require_text crates/compiler/tests/coroutine.rs 'm0025_structured_scope_diagnostics_preserve_order_and_spans'

require_absent_path crates/compiler/src/scheduler.rs

echo "m0025 structured task scope analysis validation passed"
