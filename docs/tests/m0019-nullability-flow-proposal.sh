#!/usr/bin/env sh
set -eu

fail() {
  echo "m0019-proposal: $*" >&2
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

proposal=docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md
accepted=docs/adr/ADR-0028-nullability-and-flow-typing.md
ambiguity=docs/ambiguities/M0019-nullability-and-flow-typing.md
task=docs/tasks/M0019-002-nullability-flow-proposal.md
source=crates/newlang/src/type_check.rs

require_file "$proposal"
require_file "$accepted"
require_file "$ambiguity"
require_file "$task"
require_file "$source"

require_text "$proposal" '^# ADR-0028: Nullability And Flow Typing$'
require_text "$proposal" '^Status: Draft proposal - not accepted source of truth$'
require_text "$proposal" '^## Non-Authority Notice$'
require_text "$proposal" '^## Question$'
require_text "$proposal" '^## Competing Designs$'
require_text "$proposal" '^## Trade-offs$'
require_text "$proposal" '^## Recommended Draft Choice$'
require_text "$proposal" '^## Draft Concrete Nullability And Flow Model$'
require_text "$proposal" '^## Required Accepted Content$'
require_text "$proposal" '^## Required Diagnostics$'
require_text "$proposal" '^## Explicit Draft Deferrals$'
require_text "$proposal" '^## Downstream Consequences$'
require_text "$proposal" '^## Dependencies$'
require_text "$proposal" 'No implementation may depend on this proposal until accepted'
require_text "$proposal" 'nullable misuse'
require_text "$proposal" 'smart-cast eligibility'
require_text "$proposal" 'mutation invalidation'
require_text "$proposal" 'invalid_nullable_use'
require_text "$proposal" 'invalidated_refinement'
require_text "$proposal" 'unsupported_flow_rule'
require_text "$proposal" 'ambiguous_flow_rule'
require_text "$proposal" 'does not rely on Kotlin, Rust, current parser behavior, current test behavior, or current type_check behavior as implicit authority'

require_text "$accepted" '^Status: Accepted$'
require_text "$ambiguity" 'Status: `resolved`'
require_text "$task" 'Status: `(in_progress|review|complete)`'
require_text docs/SPEC.md '^## ADR-0028: Nullability And Flow Typing$'
require_absent_text "$source" 'FlowRefinement|SmartCast|invalid_nullable_use|invalidated_refinement|unsupported_flow_rule|ambiguous_flow_rule'

echo "m0019-proposal: nullability and flow typing proposal validation passed"
