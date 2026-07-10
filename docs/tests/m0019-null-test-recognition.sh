#!/usr/bin/env sh
set -eu

fail() {
  echo "m0019-null-test-recognition: $*" >&2
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

source=crates/compiler/src/type_check.rs
tests=crates/compiler/tests/type_check.rs
task=docs/tasks/M0019-008-null-test-recognition.md

require_file "$source"
require_file "$tests"
require_file "$task"

require_text "$source" 'pub enum NullTestRefinedBranch'
require_text "$source" 'pub struct RecognizedNullTest'
require_text "$source" 'pub fn recognize_m0019_null_tests'
require_text "$source" 'ParsedBinaryOperator::Equal'
require_text "$source" 'ParsedBinaryOperator::NotEqual'
require_text "$source" 'ParsedLiteralKind::Null'
require_text "$source" 'NullTestRefinedBranch::Then'
require_text "$source" 'NullTestRefinedBranch::Else'
require_text "$tests" 'm0019_null_test_recognition_accepts_direct_not_equal_forms'
require_text "$tests" 'm0019_null_test_recognition_accepts_direct_equal_forms_as_else_refinements'
require_text "$tests" 'm0019_null_test_recognition_ignores_unsupported_condition_shapes'

require_absent_text "$source" 'apply_smart_cast|check_nullable_use|walk_if_branch'

echo "m0019-null-test-recognition: null-test recognition validation passed"
