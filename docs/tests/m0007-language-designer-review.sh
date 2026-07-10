#!/usr/bin/env sh
set -eu

fail() {
  echo "m0007-language-designer: $*" >&2
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
task=docs/tasks/M0007-004-lexical-grammar-language-designer-review.md
review=docs/adr/proposals/reviews/ADR-0021-language-designer-review.md
decision=docs/adr/proposals/reviews/ADR-0021-chief-architect-decision.md

require_file "$proposal"
require_file "$ambiguity"
require_file "$task"
require_file "$decision"
require_file "$review"
require_file docs/adr/ADR-0021-lexical-grammar.md

require_text "$proposal" '^Status: Draft proposal - not accepted source of truth$'
require_text "$ambiguity" 'Status: `resolved`'
require_text "$decision" '^Decision: approved$'
require_text docs/adr/ADR-0021-lexical-grammar.md '^Status: Accepted$'
require_text "$task" 'Milestone: `M0007`'
require_text "$task" 'Do not treat the draft proposal as accepted semantics'

require_text "$review" '^Decision: request-revision-before-acceptance$'
require_text "$review" 'Kotlin-like ergonomics'
require_text "$review" 'token classes'
require_text "$review" 'keyword policy'
require_text "$review" 'literal policy'
require_text "$review" 'operator and delimiter set'
require_text "$review" 'diagnostic-facing lexical rules'
require_text "$review" 'not accepted source of truth'
require_text "$review" 'must not implement lexer behavior'

require_absent_path crates/compiler/src/token.rs

echo "m0007-language-designer: lexical grammar ownership review validation passed"
