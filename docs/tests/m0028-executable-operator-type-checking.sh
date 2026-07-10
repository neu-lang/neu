#!/usr/bin/env sh
set -eu

fail() {
  echo "m0028-executable-operator-type-checking: $*" >&2
  exit 1
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

task=docs/tasks/M0028-002-executable-operator-type-checking.md

[ -f "$task" ] || fail "missing task file"
require_text "$task" 'Milestone: `M0028`'
require_text crates/compiler/src/parser.rs 'ParsedUnaryOperator'
require_text crates/compiler/src/parser.rs 'unary_expressions'
require_text crates/compiler/src/type_check.rs 'type_m0028_executable_int_operators'
require_text crates/compiler/tests/parser.rs 'm0028_records_executable_unary_operator_metadata'
require_text crates/compiler/tests/type_check.rs 'm0028_executable_int_operators_type_every_supported_operator'
require_text crates/compiler/tests/type_check.rs 'm0028_executable_int_operators_reject_known_non_int_operands'
require_text crates/compiler/tests/type_check.rs 'm0028_executable_int_operators_do_not_type_unknown_operands'

cargo test -p compiler --test parser --test type_check

echo "m0028 executable operator type-checking validation passed"
