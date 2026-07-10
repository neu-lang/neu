#!/usr/bin/env sh
set -eu

fail() {
  echo "m0025-suspension-borrow-analysis: $*" >&2
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

task=docs/tasks/M0025-004-suspension-borrow-analysis.md

require_file "$task"
require_file crates/compiler/src/coroutine.rs
require_file crates/compiler/tests/coroutine.rs

require_text "$task" 'Milestone: `M0025`'
require_text "$task" 'Status: `complete`'
require_text crates/compiler/src/coroutine.rs 'BorrowAcrossSuspension'
require_text crates/compiler/src/coroutine.rs 'SuspendedBorrow'
require_text crates/compiler/src/coroutine.rs 'analyze_suspended_borrows'
require_text crates/compiler/tests/coroutine.rs 'm0025_suspension_accepts_non_concurrent_same_scope_borrows'
require_text crates/compiler/tests/coroutine.rs 'm0025_suspension_reports_concurrent_frame_access'
require_text crates/compiler/tests/coroutine.rs 'm0025_suspension_reports_outliving_borrowed_value'
require_text crates/compiler/tests/coroutine.rs 'm0025_suspension_reports_both_rejection_reasons_and_preserves_order'

require_absent_path crates/compiler/src/suspension.rs
require_absent_path crates/compiler/src/scheduler.rs

echo "m0025 suspension borrow analysis validation passed"
