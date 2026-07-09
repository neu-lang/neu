#!/usr/bin/env sh
set -eu

fail() {
  echo "m0011-review: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope path exists while ADR-0022 is unaccepted: $1"
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

proposal=docs/adr/proposals/ADR-0022-declaration-syntax.md
ambiguity=docs/ambiguities/M0008-declaration-syntax.md
review_dir=docs/adr/proposals/reviews

require_file "$proposal"
require_file "$ambiguity"
require_file "$review_dir/ADR-0022-language-lawyer-review.md"
require_file "$review_dir/ADR-0022-diagnostics-review.md"
require_file "$review_dir/ADR-0022-simplicity-review.md"
require_file "$review_dir/ADR-0022-chief-architect-decision.md"

require_text "$proposal" '^Status: Draft proposal - not accepted source of truth$'
require_text "$ambiguity" 'Status: `open`'
require_text "$ambiguity" 'Blocking milestone: `M0011`'

require_text "$review_dir/ADR-0022-language-lawyer-review.md" '^Decision: request-revision-before-acceptance$'
require_text "$review_dir/ADR-0022-language-lawyer-review.md" 'package declaration ordering'
require_text "$review_dir/ADR-0022-language-lawyer-review.md" 'type grammar dependency'

require_text "$review_dir/ADR-0022-diagnostics-review.md" '^Decision: request-revision-before-acceptance$'
require_text "$review_dir/ADR-0022-diagnostics-review.md" 'declaration diagnostics'
require_text "$review_dir/ADR-0022-diagnostics-review.md" 'ADR-0015'
require_text "$review_dir/ADR-0022-diagnostics-review.md" 'primary span'

require_text "$review_dir/ADR-0022-simplicity-review.md" '^Decision: request-revision-before-acceptance$'
require_text "$review_dir/ADR-0022-simplicity-review.md" 'small Kotlin-like custom declaration grammar'
require_text "$review_dir/ADR-0022-simplicity-review.md" 'rejects adopting Kotlin wholesale'

require_text "$review_dir/ADR-0022-chief-architect-decision.md" '^Decision: pending$'
require_text "$review_dir/ADR-0022-chief-architect-decision.md" 'not accepted source of truth'
require_text "$review_dir/ADR-0022-chief-architect-decision.md" 'No acceptance yet'

require_absent_path crates/newlang/src/parser.rs
require_absent_path tests/fixtures/parser

echo "m0011-review: declaration syntax proposal review validation passed"
