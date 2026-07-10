#!/usr/bin/env sh
set -eu

fail() {
  echo "m0024-thread-capability-semantics: $*" >&2
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

adr=docs/adr/ADR-0037-bootstrap-thread-capability-analysis.md
task=docs/tasks/M0024-002-bootstrap-thread-capability-semantics.md
ambiguity=docs/ambiguities/M0024-thread-capability-semantics.md

require_file "$adr"
require_file "$task"
require_file "$ambiguity"

require_text "$adr" '^Status: Accepted$'
require_text "$adr" '`Send`'
require_text "$adr" '`Share`'
require_text "$adr" 'metadata-only bootstrap thread-capability model'
require_text "$adr" '`String` satisfies `Send` but not `Share`'
require_text "$adr" 'Current-module nominal user-defined types satisfy neither'
require_text "$adr" 'Generic parameter types satisfy neither capability'
require_text "$adr" 'boundary node and ordered capture records'
require_text "$adr" '`missing_thread_capability`'
require_text "$adr" 'M0024 must not add parser support for concurrency'
require_text "$adr" 'constructs\.'

require_text docs/SPEC.md '^## ADR-0037: Bootstrap Thread Capability Analysis$'
require_text docs/SPEC.md '`missing_thread_capability`'
require_text "$ambiguity" 'Status: `resolved`'
require_text "$ambiguity" 'Accepted `docs/adr/ADR-0037-bootstrap-thread-capability-analysis.md`'
require_text "$task" 'Status: `complete`'
require_text "$task" 'ADR-0037 accepted'

require_absent_path crates/compiler/src/concurrency.rs

echo "m0024-thread-capability-semantics: accepted source-of-truth validation passed"
