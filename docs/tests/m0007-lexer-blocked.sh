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
  [ ! -e "$1" ] || fail "out-of-scope path exists while M0007 is blocked: $1"
}

require_file docs/ambiguities/M0006-lexical-grammar.md
require_file docs/lexer/token-model.md
require_file docs/tasks/M0007-001-lexical-grammar-blocker.md

require_text docs/ambiguities/M0006-lexical-grammar.md 'Status: `open`'
require_text docs/ambiguities/M0006-lexical-grammar.md 'Detailed lexical grammar is missing'
require_text docs/ambiguities/M0006-lexical-grammar.md 'No implementation may proceed on concrete lexical behavior'
require_text docs/tasks/M0007-001-lexical-grammar-blocker.md 'Status: `blocked`'
require_text docs/tasks/M0007-001-lexical-grammar-blocker.md 'Language Designer'
require_text docs/tasks/M0007-001-lexical-grammar-blocker.md 'Chief Architect'

require_absent_path crates/newlang/src/lexer.rs
require_absent_path crates/newlang/src/token.rs
require_absent_path tests/fixtures/lexer/keywords.fixture.toml
require_absent_path tests/fixtures/lexer/identifiers.fixture.toml
require_absent_path tests/fixtures/lexer/literals.fixture.toml

echo "m0007: lexer implementation correctly blocked by lexical grammar ambiguity"
