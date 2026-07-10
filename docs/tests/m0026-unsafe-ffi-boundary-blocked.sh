#!/usr/bin/env sh
set -eu

fail() {
  echo "m0026-unsafe-ffi-boundary-blocked: $*" >&2
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

ambiguity=docs/ambiguities/M0026-unsafe-ffi-boundary-semantics.md
task=docs/tasks/M0026-001-unsafe-ffi-boundary-semantics-blocker.md

require_file "$ambiguity"
require_file "$task"

require_text "$ambiguity" 'Status: `resolved`'
require_text "$ambiguity" 'Required Owner: `main-task semantic design`'
require_text "$ambiguity" 'Which records represent unsafe contexts'
require_text "$ambiguity" 'metadata-only bootstrap subset'
require_text "$ambiguity" 'No implementation may proceed on M0026'
require_text "$ambiguity" 'Accepted `docs/adr/ADR-0039-bootstrap-unsafe-ffi-boundary-analysis.md`'
require_text "$task" 'Status: `complete`'
require_text "$task" 'An accepted ADR or spec revision must define either a source-syntax subset or a'
require_text "$task" 'No compiler implementation is added'

require_absent_path crates/compiler/src/unsafe_boundary.rs
require_absent_path crates/compiler/src/ffi.rs

echo "m0026 unsafe and FFI boundary semantics blocker resolution validation passed"
