#!/usr/bin/env sh
set -eu

fail() {
  echo "m0012-proposal: $*" >&2
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

proposal=docs/adr/proposals/ADR-0023-type-and-generic-syntax.md
ambiguity=docs/ambiguities/M0008-type-generic-syntax.md
task=docs/tasks/M0012-002-type-generic-syntax-proposal.md

require_file "$proposal"
require_file "$ambiguity"
require_file "$task"

require_text "$proposal" '^# ADR-0023: Type And Generic Syntax$'
require_text "$proposal" '^Status: Draft proposal - not accepted source of truth$'
require_text "$proposal" '^## Non-Authority Notice$'
require_text "$proposal" '^## Question$'
require_text "$proposal" '^## Competing Designs$'
require_text "$proposal" '^## Trade-offs$'
require_text "$proposal" '^## Recommended Draft Choice$'
require_text "$proposal" '^## Draft Syntax Direction$'
require_text "$proposal" '^## Required Accepted Content$'
require_text "$proposal" '^## Downstream Consequences$'
require_text "$proposal" '^## Dependencies$'
require_text "$proposal" 'No parser implementation may depend on this proposal until accepted'
require_text "$proposal" 'small Kotlin-like custom type grammar'
require_text "$proposal" 'nullable type syntax'
require_text "$proposal" 'generic parameter syntax'
require_text "$proposal" 'generic argument syntax'
require_text "$proposal" 'capability-bound syntax'
require_text "$proposal" 'function type syntax'
require_text "$proposal" 'type syntax diagnostics'
require_text "$proposal" 'not rely on Kotlin, Rust, Go, or existing compiler behavior as implicit authority'

require_text "$ambiguity" 'Status: `resolved`'
require_text "$ambiguity" 'docs/adr/ADR-0023-type-and-generic-syntax.md'
require_text "$ambiguity" 'Blocking milestone: `M0012`'
require_text "$task" 'Status: `complete`'

require_file docs/adr/ADR-0023-type-and-generic-syntax.md
require_text docs/adr/ADR-0023-type-and-generic-syntax.md '^Status: Accepted$'
require_absent_path tests/fixtures/parser/types
require_absent_path tests/fixtures/parser/generics
require_text docs/SPEC.md '^## ADR-0023: Type And Generic Syntax$'
require_absent_text crates/newlang/src/parser.rs 'parse_type|parse_generic|parse_capability|TypeRef|GenericParameter|GenericArgument|CapabilityBound|NullableType|FunctionType'
require_absent_text crates/newlang/src/ast.rs 'TypeRef|GenericParameter|GenericArgument|CapabilityBound|NullableType|FunctionType'

echo "m0012-proposal: historical proposal validation passed after ADR-0023 acceptance"
