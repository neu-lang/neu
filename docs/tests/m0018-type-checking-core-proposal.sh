#!/usr/bin/env sh
set -eu

fail() {
  echo "m0018-proposal: $*" >&2
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
  grep -Eq -- "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

require_absent_text() {
  file="$1"
  pattern="$2"
  if grep -Eq -- "$pattern" "$file"; then
    fail "unexpected pattern in $file: $pattern"
  fi
}

proposal=docs/adr/proposals/ADR-0027-type-checking-core.md
accepted=docs/adr/ADR-0027-type-checking-core.md
ambiguity=docs/ambiguities/M0018-type-checking-core.md
task=docs/tasks/M0018-002-type-checking-core-proposal.md
source=crates/compiler/src/type_check.rs

require_file "$proposal"
require_file "$accepted"
require_file "$ambiguity"
require_file "$task"
require_file "$source"

require_text "$proposal" '^# ADR-0027: Type Checking Core$'
require_text "$proposal" '^Status: Draft proposal - not accepted source of truth$'
require_text "$proposal" '^## Non-Authority Notice$'
require_text "$proposal" '^## Question$'
require_text "$proposal" '^## Competing Designs$'
require_text "$proposal" '^## Trade-offs$'
require_text "$proposal" '^## Recommended Draft Choice$'
require_text "$proposal" '^## Draft Concrete Type Checking Model$'
require_text "$proposal" '^## Required Accepted Content$'
require_text "$proposal" '^## Required Diagnostics$'
require_text "$proposal" '^## Explicit Draft Deferrals$'
require_text "$proposal" '^## Downstream Consequences$'
require_text "$proposal" '^## Dependencies$'
require_text "$proposal" 'No implementation may depend on this proposal until accepted'
require_text "$proposal" 'literal typing'
require_text "$proposal" 'primitive scalar categories'
require_text "$proposal" 'assignment compatibility'
require_text "$proposal" 'Direct function declaration calls are deferred for M0018'
require_text "$proposal" 'Structural function type application is deferred for M0018'
require_text "$proposal" 'type_mismatch'
require_text "$proposal" 'unresolved_type_rule'
require_text "$proposal" 'unsupported_type_rule'
require_text "$proposal" 'ambiguous_type_rule'
require_text "$proposal" 'not rely on Kotlin, Rust, Go, current parser behavior, current test behavior, or current type_check behavior as implicit authority'

require_text "$accepted" '^Status: Accepted$'
require_text "$ambiguity" 'Status: `resolved`'
require_text "$task" 'Status: `(review|complete)`'
require_text docs/SPEC.md '^## ADR-0027: Type Checking Core$'
require_absent_text "$source" 'check_expression|check_declaration|infer_type|literal_type|resolve_call|check_assignment|TypedExpression|TypedProgram|WellTyped'

echo "m0018-proposal: type checking core proposal validation passed"
