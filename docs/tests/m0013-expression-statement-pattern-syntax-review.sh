#!/usr/bin/env sh
set -eu

fail() {
  echo "m0013-review: $*" >&2
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
review_dir=docs/adr/proposals/reviews
task=docs/tasks/M0013-003-expression-statement-pattern-syntax-proposal-review.md

require_file "$proposal"
require_file "$ambiguity"
require_file "$task"
require_file "$review_dir/ADR-0024-language-lawyer-review.md"
require_file "$review_dir/ADR-0024-adversarial-review.md"
require_file "$review_dir/ADR-0024-diagnostics-review.md"
require_file "$review_dir/ADR-0024-simplicity-review.md"
require_file "$review_dir/ADR-0024-chief-architect-decision.md"

require_text "$proposal" '^Status: Draft proposal - not accepted source of truth$'
require_text "$ambiguity" 'Status: `resolved`'
require_text "$ambiguity" 'docs/adr/ADR-0024-expression-statement-pattern-syntax.md'
require_text "$ambiguity" 'Blocking milestone: `M0013`'

require_text "$review_dir/ADR-0024-language-lawyer-review.md" '^Decision: request-revision-before-acceptance$'
require_text "$review_dir/ADR-0024-language-lawyer-review.md" 'concrete grammar'
require_text "$review_dir/ADR-0024-language-lawyer-review.md" 'operator precedence'
require_text "$review_dir/ADR-0024-language-lawyer-review.md" 'pattern grammar'

require_text "$review_dir/ADR-0024-adversarial-review.md" '^Decision: request-revision-before-acceptance$'
require_text "$review_dir/ADR-0024-adversarial-review.md" 'ownership scope'
require_text "$review_dir/ADR-0024-adversarial-review.md" 'coroutine'
require_text "$review_dir/ADR-0024-adversarial-review.md" 'unsafe'

require_text "$review_dir/ADR-0024-diagnostics-review.md" '^Decision: request-revision-before-acceptance$'
require_text "$review_dir/ADR-0024-diagnostics-review.md" 'primary span'
require_text "$review_dir/ADR-0024-diagnostics-review.md" 'recovery action'
require_text "$review_dir/ADR-0024-diagnostics-review.md" 'safe suggestion'

require_text "$review_dir/ADR-0024-simplicity-review.md" '^Decision: request-revision-before-acceptance$'
require_text "$review_dir/ADR-0024-simplicity-review.md" 'small Kotlin-like custom body grammar'
require_text "$review_dir/ADR-0024-simplicity-review.md" 'defer'

require_text "$review_dir/ADR-0024-chief-architect-decision.md" '^Decision: approved$'
require_text "$review_dir/ADR-0024-chief-architect-decision.md" 'Accepted source of truth: `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`'
require_text "$review_dir/ADR-0024-chief-architect-decision.md" 'M0013 body parser fixture and implementation tasks may proceed'

require_text "$task" 'Status: `complete`'

require_file docs/adr/ADR-0024-expression-statement-pattern-syntax.md
require_text docs/adr/ADR-0024-expression-statement-pattern-syntax.md '^Status: Accepted$'
require_text docs/SPEC.md '^## ADR-0024: Expression Statement And Pattern Syntax$'
require_absent_text crates/newlang/src/parser.rs 'parse_expression|parse_statement|parse_pattern|parse_block|parse_when|parse_match|parse_coroutine|parse_unsafe'
require_absent_text crates/newlang/src/ast.rs 'Expression|Statement|Pattern|Block|When|Match|UnsafeBlock|Coroutine'

echo "m0013-review: expression statement and pattern syntax proposal review validation passed"
