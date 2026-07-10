#!/usr/bin/env sh
set -eu

fail() {
  echo "m0018-review: $*" >&2
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
task=docs/tasks/M0018-003-type-checking-core-proposal-review.md
source=crates/newlang/src/type_check.rs

require_file "$proposal"
require_absent_path "$accepted"
require_file "$ambiguity"
require_file "$task"
require_file "$source"

for review in \
  docs/adr/proposals/reviews/ADR-0027-language-lawyer-review.md \
  docs/adr/proposals/reviews/ADR-0027-diagnostics-review.md \
  docs/adr/proposals/reviews/ADR-0027-adversarial-review.md \
  docs/adr/proposals/reviews/ADR-0027-spec-compliance-review.md \
  docs/adr/proposals/reviews/ADR-0027-simplicity-review.md \
  docs/adr/proposals/reviews/ADR-0027-chief-architect-decision.md
do
  require_file "$review"
  require_text "$review" 'ADR-0027'
  require_text "$review" 'M0018'
done

require_text docs/adr/proposals/reviews/ADR-0027-language-lawyer-review.md 'request revision before acceptance'
require_text docs/adr/proposals/reviews/ADR-0027-language-lawyer-review.md 'typed output shape'
require_text docs/adr/proposals/reviews/ADR-0027-diagnostics-review.md 'primary span'
require_text docs/adr/proposals/reviews/ADR-0027-diagnostics-review.md 'recovery action'
require_text docs/adr/proposals/reviews/ADR-0027-diagnostics-review.md 'source-of-truth citation'
require_text docs/adr/proposals/reviews/ADR-0027-diagnostics-review.md 'safe suggestion'
require_text docs/adr/proposals/reviews/ADR-0027-adversarial-review.md 'soundness'
require_text docs/adr/proposals/reviews/ADR-0027-adversarial-review.md 'must not implement'
require_text docs/adr/proposals/reviews/ADR-0027-spec-compliance-review.md 'not accepted source of truth'
require_text docs/adr/proposals/reviews/ADR-0027-simplicity-review.md 'bootstrap subset'
require_text docs/adr/proposals/reviews/ADR-0027-chief-architect-decision.md 'Decision: pending revision'
require_text docs/adr/proposals/reviews/ADR-0027-chief-architect-decision.md 'Do not implement M0018 type checking from this proposal'

require_text "$proposal" '^Status: Draft proposal - not accepted source of truth$'
require_text "$ambiguity" 'Status: `open`'
require_text "$task" 'Status: `(review|complete)`'
require_absent_text docs/SPEC.md '^## ADR-0027: Type Checking Core$'
require_absent_text "$source" 'check_expression|check_declaration|infer_type|literal_type|resolve_call|check_assignment|TypedExpression|TypedProgram|WellTyped'

echo "m0018-review: type checking core proposal review validation passed"
