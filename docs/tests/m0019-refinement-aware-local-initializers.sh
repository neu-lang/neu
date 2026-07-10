#!/usr/bin/env sh
set -eu

fail() {
  echo "m0019-refinement-aware-local-initializers: $*" >&2
  exit 1
}

require_text() {
  grep -Eq -- "$2" "$1" || fail "missing expected pattern in $1: $2"
}

source=crates/compiler/src/type_check.rs
tests=crates/compiler/tests/type_check.rs
task=docs/tasks/M0019-015-refinement-aware-local-initializers.md

[ -f "$source" ] || fail "missing required file: $source"
[ -f "$tests" ] || fail "missing required file: $tests"
[ -f "$task" ] || fail "missing required file: $task"

require_text "$source" 'pub fn type_m0019_local_declaration_initializers'
require_text "$source" 'valid_m0019_refined_value_type'
require_text "$source" 'TypeRuleDiagnostic::NullableAssignmentWithoutRefinement'
require_text "$tests" 'm0019_refinement_aware_local_initializer_accepts_exact_active_view_and_preserves_original_records'
require_text "$tests" 'm0019_refinement_aware_local_initializer_diagnoses_exact_unrefined_name_only'
require_text "$tests" 'm0019_refinement_aware_local_initializer_preserves_nullable_compatibility'
require_text "$tests" 'm0019_refinement_aware_local_initializer_rejects_invalid_or_cross_use_views'
require_text "$tests" 'm0019_refinement_aware_local_initializer_does_not_consume_another_use_view'

echo "m0019-refinement-aware-local-initializers: test contract validated"
