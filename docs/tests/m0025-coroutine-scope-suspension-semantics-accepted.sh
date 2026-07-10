#!/usr/bin/env sh
set -eu

fail() {
  echo "m0025-coroutine-scope-suspension-semantics: $*" >&2
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

adr=docs/adr/ADR-0038-bootstrap-coroutine-scope-and-suspension-analysis.md
task=docs/tasks/M0025-002-bootstrap-coroutine-scope-suspension-semantics.md
ambiguity=docs/ambiguities/M0025-coroutine-scope-suspension-semantics.md

require_file "$adr"
require_file "$task"
require_file "$ambiguity"

require_text "$adr" '^Status: Accepted$'
require_text "$adr" 'metadata-only bootstrap coroutine scope and suspension model'
require_text "$adr" '`task_scope_escape`'
require_text "$adr" '`borrow_across_suspension`'
require_text "$adr" 'M0025 introduces no source-level coroutine'
require_text "$adr" 'completion-or-cancellation scope is'
require_text "$adr" 'the same scope as its containing structured scope'
require_text "$adr" 'suspended frame is not concurrently accessible'
require_text "$adr" 'suspended-frame scope is the same scope'
require_text "$adr" "borrowed value's lifetime"
require_text "$adr" 'Cancellation resource-safety in M0025 is limited'

require_text docs/SPEC.md '^## ADR-0038: Bootstrap Coroutine Scope And Suspension Analysis$'
require_text docs/SPEC.md '`task_scope_escape`'
require_text docs/SPEC.md '`borrow_across_suspension`'
require_text "$ambiguity" 'Status: `resolved`'
require_text "$ambiguity" 'Accepted `docs/adr/ADR-0038-bootstrap-coroutine-scope-and-suspension-analysis.md`'
require_text "$task" 'Status: `complete`'
require_text "$task" 'ADR-0038 is accepted'

require_absent_path crates/compiler/src/suspension.rs
require_absent_path crates/compiler/src/scheduler.rs

echo "m0025-coroutine-scope-suspension-semantics: accepted source-of-truth validation passed"
