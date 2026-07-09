#!/usr/bin/env sh
set -eu

fail() {
  echo "m0013-proposal: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
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

proposal=docs/adr/proposals/ADR-0024-expression-statement-pattern-syntax.md
ambiguity=docs/ambiguities/M0008-expression-statement-pattern-syntax.md
task=docs/tasks/M0013-002-expression-statement-pattern-syntax-proposal.md

require_file "$proposal"
require_file "$ambiguity"
require_file "$task"

require_text "$proposal" '^# ADR-0024: Expression Statement And Pattern Syntax$'
require_text "$proposal" '^Status: Draft proposal - not accepted source of truth$'
require_text "$proposal" '^## Non-Authority Notice$'
require_text "$proposal" '^## Question$'
require_text "$proposal" '^## Competing Designs$'
require_text "$proposal" '^## Trade-offs$'
require_text "$proposal" '^## Recommended Draft Direction$'
require_text "$proposal" '^## Required Accepted Content$'
require_text "$proposal" '^## Required Diagnostics$'
require_text "$proposal" '^## Explicit Draft Deferrals$'
require_text "$proposal" '^## Downstream Consequences$'
require_text "$proposal" '^## Dependencies$'
require_text "$proposal" 'No parser implementation may depend on this proposal until accepted'
require_text "$proposal" 'expression grammar'
require_text "$proposal" 'statement grammar'
require_text "$proposal" 'pattern grammar'
require_text "$proposal" 'operator precedence'
require_text "$proposal" 'ownership scope'
require_text "$proposal" 'coroutine syntax'
require_text "$proposal" 'unsafe block syntax'
require_text "$proposal" 'parser recovery'
require_text "$proposal" 'not rely on Kotlin, Rust, Go, or existing compiler behavior as implicit authority'

require_text "$ambiguity" 'Status: `resolved`'
require_text "$ambiguity" 'docs/adr/ADR-0024-expression-statement-pattern-syntax.md'
require_text "$ambiguity" 'Blocking milestone: `M0013`'
require_text "$task" 'Status: `complete`'

require_file docs/adr/ADR-0024-expression-statement-pattern-syntax.md
require_text docs/adr/ADR-0024-expression-statement-pattern-syntax.md '^Status: Accepted$'
require_text docs/SPEC.md '^## ADR-0024: Expression Statement And Pattern Syntax$'
require_absent_text crates/newlang/src/parser.rs 'parse_expression|parse_statement|parse_pattern|parse_block|parse_when|parse_match|parse_coroutine|parse_unsafe'
require_absent_text crates/newlang/src/ast.rs 'When|Match|UnsafeBlock|Coroutine'

echo "m0013-proposal: expression statement and pattern syntax proposal validation passed"
