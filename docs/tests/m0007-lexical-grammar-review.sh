#!/usr/bin/env sh
set -eu

fail() {
  echo "m0007-review: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope path exists before lexer implementation task: $1"
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

proposal=docs/adr/proposals/ADR-0021-lexical-grammar.md
ambiguity=docs/ambiguities/M0006-lexical-grammar.md
task=docs/tasks/M0007-003-lexical-grammar-proposal-review.md
review_dir=docs/adr/proposals/reviews

require_file "$proposal"
require_file "$ambiguity"
require_file "$task"
require_file "$review_dir/ADR-0021-adversarial-review.md"
require_file "$review_dir/ADR-0021-diagnostics-review.md"
require_file "$review_dir/ADR-0021-simplicity-review.md"
require_file "$review_dir/ADR-0021-chief-architect-decision.md"

require_text "$proposal" '^Status: Draft proposal - not accepted source of truth$'
require_text "$proposal" 'No lexer implementation may depend on this proposal until accepted'
require_text "$ambiguity" 'Status: `resolved`'
require_text "$task" 'Milestone: `M0007`'
require_text "$task" 'Do not treat the draft proposal as accepted semantics'

require_text "$review_dir/ADR-0021-adversarial-review.md" '^Decision: pass-with-required-resolution$'
require_text "$review_dir/ADR-0021-adversarial-review.md" 'identifier Unicode'
require_text "$review_dir/ADR-0021-adversarial-review.md" 'comment nesting'
require_text "$review_dir/ADR-0021-adversarial-review.md" 'string escapes'
require_text "$review_dir/ADR-0021-adversarial-review.md" 'integer literal overflow'

require_text "$review_dir/ADR-0021-diagnostics-review.md" '^Decision: pass-with-required-resolution$'
require_text "$review_dir/ADR-0021-diagnostics-review.md" 'lexical error categories'
require_text "$review_dir/ADR-0021-diagnostics-review.md" 'source spans'
require_text "$review_dir/ADR-0021-diagnostics-review.md" 'ADR-0015'

require_text "$review_dir/ADR-0021-simplicity-review.md" '^Decision: pass-with-required-resolution$'
require_text "$review_dir/ADR-0021-simplicity-review.md" 'small Kotlin-like custom lexical grammar'
require_text "$review_dir/ADR-0021-simplicity-review.md" 'rejects adopting Kotlin wholesale'

require_text "$review_dir/ADR-0021-chief-architect-decision.md" '^Decision: approved$'
require_text "$review_dir/ADR-0021-chief-architect-decision.md" 'Accepted `docs/adr/ADR-0021-lexical-grammar.md` as source of truth'
require_text docs/adr/ADR-0021-lexical-grammar.md '^Status: Accepted$'

require_absent_path crates/newlang/src/lexer.rs
require_absent_path crates/newlang/src/token.rs
require_absent_path tests/fixtures/lexer/keywords.fixture.toml
require_absent_path tests/fixtures/lexer/identifiers.fixture.toml
require_absent_path tests/fixtures/lexer/literals.fixture.toml

echo "m0007-review: lexical grammar proposal review and acceptance validation passed"
