#!/usr/bin/env sh
set -eu

fail() {
  echo "m0016-review: $*" >&2
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

proposal=docs/adr/proposals/ADR-0026-name-resolution-policy.md
ambiguity=docs/ambiguities/M0016-name-resolution-policy.md
review_dir=docs/adr/proposals/reviews
task=docs/tasks/M0016-003-name-resolution-policy-proposal-review.md

require_file "$proposal"
require_file "$ambiguity"
require_file "$task"
require_file "$review_dir/ADR-0026-language-lawyer-review.md"
require_file "$review_dir/ADR-0026-diagnostics-review.md"
require_file "$review_dir/ADR-0026-adversarial-review.md"
require_file "$review_dir/ADR-0026-spec-compliance-review.md"
require_file "$review_dir/ADR-0026-simplicity-review.md"
require_file "$review_dir/ADR-0026-chief-architect-decision.md"

require_text "$proposal" '^Status: Draft proposal - not accepted source of truth$'
require_text "$ambiguity" 'Status: `open`'
require_text "$ambiguity" 'Blocking milestone: `M0016`'

require_text "$review_dir/ADR-0026-language-lawyer-review.md" '^Decision: request-revision-before-acceptance$'
require_text "$review_dir/ADR-0026-language-lawyer-review.md" 'exact AST node kinds'
require_text "$review_dir/ADR-0026-language-lawyer-review.md" 'pattern bindings'
require_text "$review_dir/ADR-0026-language-lawyer-review.md" 'declaration order'

require_text "$review_dir/ADR-0026-diagnostics-review.md" '^Decision: request-revision-before-acceptance$'
require_text "$review_dir/ADR-0026-diagnostics-review.md" 'primary span'
require_text "$review_dir/ADR-0026-diagnostics-review.md" 'recovery action'
require_text "$review_dir/ADR-0026-diagnostics-review.md" 'safe suggestion'

require_text "$review_dir/ADR-0026-adversarial-review.md" '^Decision: request-revision-before-acceptance$'
require_text "$review_dir/ADR-0026-adversarial-review.md" 'shadowing'
require_text "$review_dir/ADR-0026-adversarial-review.md" 'duplicate'
require_text "$review_dir/ADR-0026-adversarial-review.md" 'ambiguity'

require_text "$review_dir/ADR-0026-spec-compliance-review.md" '^Decision: request-revision-before-acceptance$'
require_text "$review_dir/ADR-0026-spec-compliance-review.md" 'accepted source of truth'
require_text "$review_dir/ADR-0026-spec-compliance-review.md" 'non-authoritative'

require_text "$review_dir/ADR-0026-simplicity-review.md" '^Decision: request-revision-before-acceptance$'
require_text "$review_dir/ADR-0026-simplicity-review.md" 'bootstrap subset'
require_text "$review_dir/ADR-0026-simplicity-review.md" 'defer'

require_text "$review_dir/ADR-0026-chief-architect-decision.md" '^Decision: pending-revision$'
require_text "$review_dir/ADR-0026-chief-architect-decision.md" 'ADR-0026 is not accepted'
require_text "$review_dir/ADR-0026-chief-architect-decision.md" 'M0016 remains blocked'

require_text "$task" 'Status: `complete`'

require_absent_text docs/SPEC.md '^## ADR-0026: Name Resolution Policy$'
require_absent_text crates/newlang/src/lib.rs 'pub mod name_resolution|pub mod resolution'
require_absent_text crates/newlang/src/parser.rs 'NameResolution|UnresolvedName|ResolvedName|ImportResolver'

echo "m0016-review: name resolution policy proposal review validation passed"
