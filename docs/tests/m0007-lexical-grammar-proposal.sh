#!/usr/bin/env sh
set -eu

fail() {
  echo "m0007-proposal: $*" >&2
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

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope path exists before lexer implementation task: $1"
}

require_file docs/adr/proposals/ADR-0021-lexical-grammar.md
require_file docs/adr/ADR-0021-lexical-grammar.md
require_file docs/tasks/M0007-002-lexical-grammar-proposal.md

require_text docs/adr/proposals/ADR-0021-lexical-grammar.md '^Status: Draft proposal - not accepted source of truth$'
require_text docs/adr/proposals/ADR-0021-lexical-grammar.md '^# ADR-0021: Lexical Grammar$'
require_text docs/adr/proposals/ADR-0021-lexical-grammar.md '^## Question$'
require_text docs/adr/proposals/ADR-0021-lexical-grammar.md '^## Competing Designs$'
require_text docs/adr/proposals/ADR-0021-lexical-grammar.md '^## Trade-offs$'
require_text docs/adr/proposals/ADR-0021-lexical-grammar.md '^## Recommended Draft Choice$'
require_text docs/adr/proposals/ADR-0021-lexical-grammar.md '^## Downstream Consequences$'
require_text docs/adr/proposals/ADR-0021-lexical-grammar.md '^## Dependencies$'
require_text docs/adr/proposals/ADR-0021-lexical-grammar.md '^## Non-Authority Notice$'
require_text docs/adr/proposals/ADR-0021-lexical-grammar.md 'docs/ambiguities/M0006-lexical-grammar.md'
require_text docs/adr/proposals/ADR-0021-lexical-grammar.md 'main task'
require_text docs/adr/proposals/ADR-0021-lexical-grammar.md 'No lexer implementation may depend on this proposal until accepted'

require_text docs/adr/ADR-0021-lexical-grammar.md '^Status: Accepted$'
require_text docs/ambiguities/M0006-lexical-grammar.md 'Status: `resolved`'

require_absent_path crates/compiler/src/token.rs

echo "m0007-proposal: lexical grammar ADR proposal and acceptance validation passed"
