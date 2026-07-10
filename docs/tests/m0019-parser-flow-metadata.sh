#!/usr/bin/env sh
set -eu

fail() {
  echo "m0019-parser-flow-metadata: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq -- "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

require_absent_text() {
  file="$1"
  pattern="$2"
  if grep -Eq -- "$pattern" "$file"; then
    fail "unexpected pattern in $file: $pattern"
  fi
}

parser=crates/compiler/src/parser.rs
tests=crates/compiler/tests/parser.rs
task=docs/tasks/M0019-007-parser-flow-metadata.md

require_file "$parser"
require_file "$tests"
require_file "$task"

require_text "$parser" 'pub struct ParsedBinaryExpression'
require_text "$parser" 'pub enum ParsedBinaryOperator'
require_text "$parser" 'pub struct ParsedIfExpression'
require_text "$parser" 'pub binary_expressions: Vec<ParsedBinaryExpression>'
require_text "$parser" 'pub if_expressions: Vec<ParsedIfExpression>'
require_text "$parser" 'operator: ParsedBinaryOperator'
require_text "$parser" 'then_block: AstNodeId'
require_text "$parser" 'else_block: Option<AstNodeId>'
require_text "$parser" 'record_binary_expression'
require_text "$parser" 'record_if_expression'
require_text "$tests" 'm0019_records_binary_expression_metadata_for_flow_inputs'
require_text "$tests" 'm0019_records_if_expression_condition_and_branch_metadata'
require_text "$tests" 'm0019_records_if_expression_without_else_as_none'

require_absent_text "$parser" 'recognize_null_test|apply_smart_cast|check_nullable_use|FlowRefinement'
require_absent_text crates/compiler/src/type_check.rs 'recognize_null_test|apply_smart_cast|check_nullable_use|walk_if_branch'

echo "m0019-parser-flow-metadata: parser flow metadata validation passed"
