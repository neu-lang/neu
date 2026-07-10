#!/usr/bin/env sh
set -eu

fail() {
  echo "m0026-unsafe-context-analysis: $*" >&2
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

task=docs/tasks/M0026-003-unsafe-context-analysis.md

require_file "$task"
require_file crates/compiler/src/unsafe_boundary.rs
require_file crates/compiler/tests/unsafe_boundary.rs

require_text "$task" 'Milestone: `M0026`'
require_text "$task" 'Status: `complete`'
require_text crates/compiler/src/lib.rs 'pub mod unsafe_boundary;'
require_text crates/compiler/src/unsafe_boundary.rs 'UnsafeOperationOutsideContext'
require_text crates/compiler/src/unsafe_boundary.rs 'analyze_unsafe_operations'
require_text crates/compiler/tests/unsafe_boundary.rs 'm0026_unsafe_analysis_accepts_proven_safe_operations_without_context'
require_text crates/compiler/tests/unsafe_boundary.rs 'm0026_unsafe_analysis_accepts_trusted_operations_in_matching_context'
require_text crates/compiler/tests/unsafe_boundary.rs 'm0026_unsafe_analysis_reports_trusted_operation_without_context'
require_text crates/compiler/tests/unsafe_boundary.rs 'm0026_unsafe_analysis_reports_non_matching_context_and_preserves_order'

require_absent_path crates/compiler/src/ffi.rs

echo "m0026 unsafe context analysis validation passed"
