#!/usr/bin/env sh
set -eu

fail() {
  echo "m0019-null-test-eligibility: $*" >&2
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
task=docs/tasks/M0019-009-null-test-eligibility.md

require_file "$source"
require_file "$tests"
require_file "$task"

require_text "$source" 'pub struct EligibleNullTestRefinement'
require_text "$source" 'pub fn select_m0019_eligible_null_tests'
require_text "$source" 'LocalBindingKind::Immutable'
require_text "$source" 'LocalBindingKind::Var'
require_text "$source" 'TypeKind::Nullable'
require_text "$source" 'TypeRuleDiagnostic::MutableLocalRefinementDeferred'
require_text "$source" 'TypeRuleDiagnostic::AmbiguousLocalBindingFlow'
require_text "$tests" 'm0019_null_test_eligibility_accepts_immutable_nullable_local'
require_text "$tests" 'm0019_null_test_eligibility_rejects_mutable_local_with_flow_diagnostic'
require_text "$tests" 'm0019_null_test_eligibility_ignores_non_nullable_and_incomplete_inputs'
require_text "$tests" 'm0019_null_test_eligibility_reports_ambiguous_local_binding_match'

require_absent_text "$source" 'record_refinement\(.*EligibleNullTestRefinement|record_refined_expression_type\(.*EligibleNullTestRefinement|apply_smart_cast|walk_if_branch'

echo "m0019-null-test-eligibility: null-test eligibility validation passed"
