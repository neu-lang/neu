#!/usr/bin/env sh
set -eu

fail() {
  echo "m0016-proposal: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope path exists during proposal task: $1"
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

require_absent_text() {
  file="$1"
  pattern="$2"
  if grep -Eq "$pattern" "$file"; then
    fail "unexpected pattern in $file: $pattern"
  fi
}

proposal=docs/adr/proposals/ADR-0026-name-resolution-policy.md
ambiguity=docs/ambiguities/M0016-name-resolution-policy.md
task=docs/tasks/M0016-002-name-resolution-policy-proposal.md

require_file "$proposal"
require_file "$ambiguity"
require_file "$task"

require_text "$proposal" '^# ADR-0026: Name Resolution Policy$'
require_text "$proposal" '^Status: Draft proposal - not accepted source of truth$'
require_text "$proposal" '^## Non-Authority Notice$'
require_text "$proposal" '^## Question$'
require_text "$proposal" '^## Competing Designs$'
require_text "$proposal" '^## Trade-offs$'
require_text "$proposal" '^## Recommended Draft Choice$'
require_text "$proposal" '^## Draft Bootstrap Resolution Subset$'
require_text "$proposal" '^## Required Accepted Content$'
require_text "$proposal" '^## Required Diagnostics$'
require_text "$proposal" '^## Explicit Draft Deferrals$'
require_text "$proposal" '^## Downstream Consequences$'
require_text "$proposal" '^## Dependencies$'
require_text "$proposal" 'No implementation may depend on this proposal until accepted'
require_text "$proposal" 'lookup order'
require_text "$proposal" 'scope boundaries'
require_text "$proposal" 'duplicate-name behavior'
require_text "$proposal" 'unresolved-name diagnostics'
require_text "$proposal" 'import semantics'
require_text "$proposal" 'not rely on Kotlin, Rust, Go, existing name-table behavior, or parser behavior as implicit authority'

require_text "$proposal" 'unresolved_name'
require_text "$proposal" 'duplicate_name'
require_text "$proposal" 'ambiguous_name'
require_text "$proposal" 'inaccessible_name'

require_text "$ambiguity" 'Status: `open`'
require_text "$ambiguity" '\[x\] Language Designer drafts a name-resolution policy ADR or `docs/SPEC.md` revision'
require_text "$task" 'Status: `complete`'

require_absent_path docs/adr/ADR-0026-name-resolution-policy.md
require_absent_text docs/SPEC.md '^## ADR-0026: Name Resolution Policy$'
require_absent_path crates/newlang/src/name_resolution.rs
require_absent_path crates/newlang/src/resolution.rs
require_absent_text crates/newlang/src/lib.rs 'pub mod name_resolution|pub mod resolution'

echo "m0016-proposal: name resolution policy proposal validation passed"
