#!/usr/bin/env sh
set -eu

fail() {
  echo "m0007: $*" >&2
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

require_file docs/adr/ADR-0021-lexical-grammar.md
require_file docs/ambiguities/M0006-lexical-grammar.md
require_file docs/lexer/token-model.md
require_file docs/tasks/M0007-001-lexical-grammar-blocker.md

require_text docs/adr/ADR-0021-lexical-grammar.md '^Status: Accepted$'
require_text docs/ambiguities/M0006-lexical-grammar.md 'Status: `resolved`'
require_text docs/ambiguities/M0006-lexical-grammar.md 'Detailed lexical grammar is missing'
require_text docs/ambiguities/M0006-lexical-grammar.md 'Accepted `docs/adr/ADR-0021-lexical-grammar.md`'
require_text docs/tasks/M0007-001-lexical-grammar-blocker.md 'Status: `blocked`'
require_text docs/tasks/M0007-001-lexical-grammar-blocker.md 'Language Designer'
require_text docs/tasks/M0007-001-lexical-grammar-blocker.md 'Chief Architect'

require_absent_path crates/newlang/src/token.rs

echo "m0007: historical lexer blocker resolved by accepted lexical grammar ADR"
