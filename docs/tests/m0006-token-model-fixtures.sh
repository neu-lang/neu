#!/usr/bin/env sh
set -eu

fail() {
  echo "m0006: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_dir() {
  [ -d "$1" ] || fail "missing required directory: $1"
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope path exists for M0006: $1"
}

require_absent_text() {
  file="$1"
  pattern="$2"
  if grep -Eq "$pattern" "$file"; then
    fail "unexpected pattern in $file: $pattern"
  fi
}

require_file docs/lexer/token-model.md
require_file docs/ambiguities/M0006-lexical-grammar.md
require_dir tests/fixtures/lexer
require_file tests/fixtures/lexer/M0006-inert.fixture.toml

require_text docs/lexer/token-model.md 'docs/SPEC\.md'
require_text docs/lexer/token-model.md 'docs/adr/'
require_text docs/lexer/token-model.md 'specified'
require_text docs/lexer/token-model.md 'blocked'
require_text docs/lexer/token-model.md 'deferred'
require_text docs/lexer/token-model.md 'No token category may be implemented from Kotlin precedent alone'

require_text docs/ambiguities/M0006-lexical-grammar.md 'Detailed lexical grammar is missing'
require_text docs/ambiguities/M0006-lexical-grammar.md 'Language Designer'
require_text docs/ambiguities/M0006-lexical-grammar.md 'M0007'
require_text docs/ambiguities/M0006-lexical-grammar.md 'guessing'

require_text tests/fixtures/lexer/M0006-inert.fixture.toml '^kind = "inert-lexer-fixture"$'
require_text tests/fixtures/lexer/M0006-inert.fixture.toml '^milestone = "M0006"$'
require_text tests/fixtures/lexer/M0006-inert.fixture.toml '^compiler_behavior = "none"$'
require_text tests/fixtures/lexer/M0006-inert.fixture.toml '^source = "docs/milestones/M0006-token-model-and-lexer-fixtures.md"$'
require_text tests/fixtures/lexer/M0006-inert.fixture.toml '^lexical_grammar_status = "blocked"$'

require_absent_text tests/fixtures/lexer/M0006-inert.fixture.toml 'source_text|expected_tokens|token_stream|keyword|identifier|literal|operator|delimiter'

require_absent_path crates/newlang/src/token.rs
require_absent_path crates/newlang/src/parser.rs

echo "m0006: token model and lexer fixture metadata validation passed"
