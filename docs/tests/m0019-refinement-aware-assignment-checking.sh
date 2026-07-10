#!/usr/bin/env sh
set -eu

fail() {
  echo "m0019-refinement-aware-assignment-checking: $*" >&2
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

source=crates/compiler/src/type_check.rs
tests=crates/compiler/tests/type_check.rs
task=docs/tasks/M0019-013-refinement-aware-assignment-checking.md

require_file "$source"
require_file "$tests"
require_file "$task"

require_text "$source" 'pub fn type_m0019_assignment_statements'
require_text "$source" 'RefinedExpressionType'
require_text "$source" 'TypeRuleDiagnostic::NullableAssignmentWithoutRefinement'
require_text "$source" 'TypeCheckDiagnostic::invalid_nullable_use'
require_text "$tests" 'm0019_refinement_aware_assignment_accepts_valid_refined_value'
require_text "$tests" 'm0019_refinement_aware_assignment_reports_unrefined_nullable_to_base'
require_text "$tests" 'm0019_refinement_aware_assignment_preserves_m0018_compatibility'
require_text "$tests" 'm0019_refinement_aware_assignment_keeps_other_mismatches'
require_text "$tests" 'm0019_refinement_aware_assignment_ignores_inconsistent_refined_views'
require_text "$tests" 'm0019_refinement_aware_assignment_rejects_duplicate_refined_views'
require_text "$tests" 'm0019_refinement_aware_assignment_rejects_forged_out_of_region_view'

echo "m0019-refinement-aware-assignment-checking: nullable assignment validation passed"
