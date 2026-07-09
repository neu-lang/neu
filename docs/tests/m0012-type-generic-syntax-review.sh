#!/usr/bin/env sh
set -eu

fail() {
  echo "m0012-review: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope path exists during review task: $1"
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
review_dir=docs/adr/proposals/reviews
task=docs/tasks/M0012-003-type-generic-syntax-proposal-review.md

require_file "$proposal"
require_file "$ambiguity"
require_file "$task"
require_file "$review_dir/ADR-0023-language-lawyer-review.md"
require_file "$review_dir/ADR-0023-adversarial-review.md"
require_file "$review_dir/ADR-0023-diagnostics-review.md"
require_file "$review_dir/ADR-0023-simplicity-review.md"
require_file "$review_dir/ADR-0023-chief-architect-decision.md"

require_text "$proposal" '^Status: Draft proposal - not accepted source of truth$'
require_text "$ambiguity" 'Status: `resolved`'
require_text "$ambiguity" 'docs/adr/ADR-0023-type-and-generic-syntax.md'
require_text "$ambiguity" 'Blocking milestone: `M0012`'

require_text "$review_dir/ADR-0023-language-lawyer-review.md" '^Decision: request-revision-before-acceptance$'
require_text "$review_dir/ADR-0023-language-lawyer-review.md" 'named type reference grammar'
require_text "$review_dir/ADR-0023-language-lawyer-review.md" 'nullable marker associativity'
require_text "$review_dir/ADR-0023-language-lawyer-review.md" 'generic parameter list placement'

require_text "$review_dir/ADR-0023-adversarial-review.md" '^Decision: request-revision-before-acceptance$'
require_text "$review_dir/ADR-0023-adversarial-review.md" 'capability-bound syntax'
require_text "$review_dir/ADR-0023-adversarial-review.md" 'variance'
require_text "$review_dir/ADR-0023-adversarial-review.md" 'borrow'

require_text "$review_dir/ADR-0023-diagnostics-review.md" '^Decision: request-revision-before-acceptance$'
require_text "$review_dir/ADR-0023-diagnostics-review.md" 'type syntax diagnostics'
require_text "$review_dir/ADR-0023-diagnostics-review.md" 'primary span'
require_text "$review_dir/ADR-0023-diagnostics-review.md" 'recovery action'

require_text "$review_dir/ADR-0023-simplicity-review.md" '^Decision: request-revision-before-acceptance$'
require_text "$review_dir/ADR-0023-simplicity-review.md" 'small Kotlin-like custom type grammar'
require_text "$review_dir/ADR-0023-simplicity-review.md" 'defer'

require_text "$review_dir/ADR-0023-chief-architect-decision.md" '^Decision: approved$'
require_text "$review_dir/ADR-0023-chief-architect-decision.md" 'Accepted source of truth: `docs/adr/ADR-0023-type-and-generic-syntax.md`'
require_text "$review_dir/ADR-0023-chief-architect-decision.md" 'M0012 type and generic parser fixture and implementation tasks may proceed'

require_text "$task" 'Status: `complete`'

require_file docs/adr/ADR-0023-type-and-generic-syntax.md
require_text docs/adr/ADR-0023-type-and-generic-syntax.md '^Status: Accepted$'
require_absent_path tests/fixtures/parser/types
require_absent_path tests/fixtures/parser/generics
require_text docs/SPEC.md '^## ADR-0023: Type And Generic Syntax$'
require_absent_text crates/newlang/src/parser.rs 'parse_type|parse_generic|parse_capability|TypeRef|GenericParameter|GenericArgument|CapabilityBound|NullableType|FunctionType'
require_absent_text crates/newlang/src/ast.rs 'TypeRef|GenericParameter|GenericArgument|CapabilityBound|NullableType|FunctionType'

echo "m0012-review: type and generic syntax review history and approval validation passed"
