#!/usr/bin/env sh
set -eu

fail() {
  echo "m0019-branch-refinement-records: $*" >&2
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

source=crates/newlang/src/type_check.rs
tests=crates/newlang/tests/type_check.rs
task=docs/tasks/M0019-010-branch-refinement-records.md

require_file "$source"
require_file "$tests"
require_file "$task"

require_text "$source" 'pub fn record_m0019_branch_refinements'
require_text "$source" 'ParsedIfExpression'
require_text "$source" 'NullTestRefinedBranch::Then'
require_text "$source" 'NullTestRefinedBranch::Else'
require_text "$source" 'record_refinement'
require_text "$source" 'RefinementRecord::new'
require_text "$tests" 'm0019_branch_refinement_records_then_branch_for_not_equal_tests'
require_text "$tests" 'm0019_branch_refinement_records_else_branch_for_equal_tests'
require_text "$tests" 'm0019_branch_refinement_skips_missing_else_and_non_condition_tests'

require_absent_text "$source" 'record_refined_expression_type\(.*record_m0019_branch_refinements|apply_smart_cast|check_nullable_use|walk_if_branch'

echo "m0019-branch-refinement-records: branch refinement record validation passed"
