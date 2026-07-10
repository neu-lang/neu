#!/usr/bin/env sh
set -eu

fail() {
  echo "m0007-fixtures: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope implementation path exists during fixture task: $1"
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

adr=docs/adr/ADR-0021-lexical-grammar.md
token_model=docs/lexer/token-model.md

require_file "$adr"
require_file "$token_model"
require_text "$adr" '^Status: Accepted$'
require_text "$token_model" 'ADR-0021'
require_absent_text "$token_model" 'Do not assume Kotlin comment syntax'
require_absent_text "$token_model" 'Concrete lexer fixtures are blocked'

for fixture in \
  tests/fixtures/lexer/keywords.fixture.toml \
  tests/fixtures/lexer/identifiers.fixture.toml \
  tests/fixtures/lexer/literals.fixture.toml \
  tests/fixtures/lexer/comments.fixture.toml \
  tests/fixtures/lexer/operators-delimiters.fixture.toml \
  tests/fixtures/lexer/errors.fixture.toml
do
  require_file "$fixture"
  require_text "$fixture" '^kind = "lexer-fixture"$'
  require_text "$fixture" '^milestone = "M0007"$'
  require_text "$fixture" '^source = "docs/adr/ADR-0021-lexical-grammar.md"$'
  require_absent_text "$fixture" 'Kotlin'
  require_absent_text "$fixture" 'parser_precedence|ast|hir|mir|backend'
done

require_text tests/fixtures/lexer/keywords.fixture.toml 'expected_tokens = \["KW_FUN", "IDENTIFIER", "LEFT_PAREN", "RIGHT_PAREN", "LEFT_BRACE", "KW_RETURN", "KW_TRUE", "RIGHT_BRACE"\]'
require_text tests/fixtures/lexer/identifiers.fixture.toml 'source_text = "alpha _beta value42"'
require_text tests/fixtures/lexer/identifiers.fixture.toml 'diagnostic = "unsupported_identifier_character"'
require_text tests/fixtures/lexer/literals.fixture.toml 'source_text = "0 42 0b1010 0x2A 1_000"'
require_text tests/fixtures/lexer/literals.fixture.toml 'Integer literal overflow is semantic, not lexical'
require_text tests/fixtures/lexer/comments.fixture.toml 'source_text = "/\* outer /\* inner \*/ done \*/ const"'
require_text tests/fixtures/lexer/operators-delimiters.fixture.toml 'expected_tokens = \["PLUS_PLUS", "MINUS_MINUS"'
require_text tests/fixtures/lexer/errors.fixture.toml 'unterminated_block_comment'
require_text tests/fixtures/lexer/errors.fixture.toml 'invalid_string_escape'
require_text tests/fixtures/lexer/errors.fixture.toml 'malformed_integer_literal'

require_absent_path crates/newlang/src/token.rs

echo "m0007-fixtures: concrete lexer fixture validation passed"
