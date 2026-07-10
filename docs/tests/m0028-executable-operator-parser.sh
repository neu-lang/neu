#!/usr/bin/env sh
set -eu

fail() {
  echo "m0028-executable-operator-parser: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope implementation path exists: $1"
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

task=docs/tasks/M0028-001-executable-operator-parser.md

require_file "$task"
require_text "$task" 'Milestone: `M0028`'
require_text "$task" 'Status: `complete`'
require_text crates/compiler/src/lexer.rs 'StarStar'
require_text crates/compiler/src/lexer.rs 'LessLess'
require_text crates/compiler/src/lexer.rs 'GreaterGreater'
require_text crates/compiler/src/lexer.rs 'Tilde'
require_text crates/compiler/src/lexer.rs 'Caret'
require_text crates/compiler/src/parser.rs 'Exponent'
require_text crates/compiler/src/parser.rs 'ShiftLeft'
require_text crates/compiler/src/parser.rs 'BitwiseXor'
require_text crates/compiler/tests/lexer.rs 'm0028_lexes_executable_operator_tokens'
require_text crates/compiler/tests/parser.rs 'm0028_records_executable_binary_operator_metadata'
require_text crates/compiler/tests/parser.rs 'm0028_parses_executable_unary_operators'
require_text crates/compiler/tests/parser.rs 'm0028_parses_exponentiation_right_associatively'

require_absent_path crates/compiler/src/hir.rs
require_absent_path crates/compiler/src/mir.rs
require_absent_path crates/compiler/src/backend

echo "m0028 executable operator parser validation passed"
