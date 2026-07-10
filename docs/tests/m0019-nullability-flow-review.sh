#!/usr/bin/env sh
set -eu

fail() {
  echo "m0019-review: $*" >&2
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

proposal=docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md
accepted=docs/adr/ADR-0028-nullability-and-flow-typing.md
ambiguity=docs/ambiguities/M0019-nullability-and-flow-typing.md
task=docs/tasks/M0019-003-nullability-flow-proposal-review.md
source=crates/newlang/src/type_check.rs

require_file "$proposal"
require_file "$accepted"
require_file "$ambiguity"
require_file "$task"
require_file "$source"

for review in \
  docs/adr/proposals/reviews/ADR-0028-language-lawyer-review.md \
  docs/adr/proposals/reviews/ADR-0028-diagnostics-review.md \
  docs/adr/proposals/reviews/ADR-0028-adversarial-review.md \
  docs/adr/proposals/reviews/ADR-0028-spec-compliance-review.md \
  docs/adr/proposals/reviews/ADR-0028-simplicity-review.md \
  docs/adr/proposals/reviews/ADR-0028-chief-architect-decision.md
do
  require_file "$review"
  require_text "$review" 'ADR-0028'
  require_text "$review" 'M0019'
done

require_text docs/adr/proposals/reviews/ADR-0028-language-lawyer-review.md 'request revision before acceptance'
require_text docs/adr/proposals/reviews/ADR-0028-language-lawyer-review.md 'branch region boundaries'
require_text docs/adr/proposals/reviews/ADR-0028-diagnostics-review.md 'primary span'
require_text docs/adr/proposals/reviews/ADR-0028-diagnostics-review.md 'recovery action'
require_text docs/adr/proposals/reviews/ADR-0028-diagnostics-review.md 'source-of-truth citation'
require_text docs/adr/proposals/reviews/ADR-0028-diagnostics-review.md 'safe suggestion'
require_text docs/adr/proposals/reviews/ADR-0028-adversarial-review.md 'soundness'
require_text docs/adr/proposals/reviews/ADR-0028-adversarial-review.md 'must not implement'
require_text docs/adr/proposals/reviews/ADR-0028-spec-compliance-review.md 'not accepted source of truth'
require_text docs/adr/proposals/reviews/ADR-0028-simplicity-review.md 'narrow subset'
require_text docs/adr/proposals/reviews/ADR-0028-chief-architect-decision.md 'Decision: approved'
require_text docs/adr/proposals/reviews/ADR-0028-chief-architect-decision.md 'Implementation may proceed only against the accepted ADR-0028 model'

require_text "$proposal" '^Status: Draft proposal - not accepted source of truth$'
require_text "$accepted" '^Status: Accepted$'
require_text "$ambiguity" 'Status: `resolved`'
require_text "$task" 'Status: `(in_progress|review|complete)`'
require_text docs/SPEC.md '^## ADR-0028: Nullability And Flow Typing$'
require_absent_text "$source" 'FlowRefinement|SmartCast|invalid_nullable_use|invalidated_refinement|unsupported_flow_rule|ambiguous_flow_rule'

echo "m0019-review: nullability and flow typing proposal review validation passed"
