#!/usr/bin/env sh
set -eu

fail() {
  echo "m0026-unsafe-ffi-boundary-semantics: $*" >&2
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

adr=docs/adr/ADR-0039-bootstrap-unsafe-ffi-boundary-analysis.md
task=docs/tasks/M0026-002-bootstrap-unsafe-ffi-boundary-semantics.md
ambiguity=docs/ambiguities/M0026-unsafe-ffi-boundary-semantics.md

require_file "$adr"
require_file "$task"
require_file "$ambiguity"

require_text "$adr" '^Status: Accepted$'
require_text "$adr" 'metadata-only bootstrap unsafe and FFI boundary model'
require_text "$adr" '`ProvenSafe`'
require_text "$adr" '`TrustedUnsafe`'
require_text "$adr" '`unsafe_operation_outside_context`'
require_text "$adr" '`missing_ffi_safety_metadata`'
require_text "$adr" 'M0026 validates metadata presence only'
require_text "$adr" 'Safe-code guarantees remain intact'

require_text docs/SPEC.md '^## ADR-0039: Bootstrap Unsafe FFI Boundary Analysis$'
require_text docs/SPEC.md '`unsafe_operation_outside_context`'
require_text docs/SPEC.md '`missing_ffi_safety_metadata`'
require_text "$ambiguity" 'Status: `resolved`'
require_text "$ambiguity" 'Accepted `docs/adr/ADR-0039-bootstrap-unsafe-ffi-boundary-analysis.md`'
require_text "$task" 'Status: `complete`'
require_text "$task" 'ADR-0039 is accepted'

require_absent_path crates/compiler/src/unsafe_boundary.rs
require_absent_path crates/compiler/src/ffi.rs

echo "m0026 unsafe and FFI boundary semantics accepted validation passed"
