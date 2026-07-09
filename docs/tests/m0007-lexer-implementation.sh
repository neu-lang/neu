#!/usr/bin/env sh
set -eu

fail() {
  echo "m0007-lexer-impl: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope path exists during lexer implementation: $1"
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

require_file docs/adr/ADR-0021-lexical-grammar.md
require_file crates/newlang/src/lexer.rs
require_file crates/newlang/tests/lexer.rs
require_file docs/tasks/M0007-008-lexer-implementation.md

require_text docs/adr/ADR-0021-lexical-grammar.md '^Status: Accepted$'
require_text crates/newlang/src/lib.rs '^pub mod lexer;$'
require_text crates/newlang/src/lexer.rs 'pub fn lex'
require_text crates/newlang/src/lexer.rs 'enum TokenKind'
require_text crates/newlang/src/lexer.rs 'enum DiagnosticKind'
require_text crates/newlang/tests/lexer.rs 'integer_overflow'
require_text crates/newlang/tests/lexer.rs 'UnsupportedIdentifierCharacter'
require_text crates/newlang/tests/lexer.rs 'DotDotLess'

require_absent_path crates/newlang/src/token.rs
require_absent_path crates/newlang/src/parser.rs
require_absent_path crates/newlang/src/ast.rs
require_absent_path crates/newlang/src/hir.rs
require_absent_path crates/newlang/src/mir.rs
require_absent_path crates/newlang/src/backend

echo "m0007-lexer-impl: lexer implementation validation passed"
