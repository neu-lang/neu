#!/usr/bin/env sh
set -eu

fail() {
  echo "m0019-region-exit-refinement-invalidation: $*" >&2
  exit 1
}

require_text() {
  grep -Eq -- "$2" "$1" || fail "missing expected pattern in $1: $2"
}

source=crates/newlang/src/type_check.rs
tests=crates/newlang/tests/type_check.rs
task=docs/tasks/M0019-016-mutation-invalidation.md

[ -f "$source" ] || fail "missing required file: $source"
[ -f "$tests" ] || fail "missing required file: $tests"
[ -f "$task" ] || fail "missing required file: $task"

require_text "$source" 'pub fn type_m0019_region_exit_refinement_invalidations'
require_text "$source" 'TypeRuleDiagnostic::RegionExitInvalidatedRefinement'
require_text "$source" 'TypeCheckDiagnostic::invalidated_refinement'
require_text "$tests" 'm0019_mutation_invalidation_classifies_only_exact_post_region_bare_name_initializer'
require_text "$tests" 'TypeRuleDiagnostic::RegionExitInvalidatedRefinement'
require_text "$tests" 'TypeCheckDiagnosticKind::InvalidNullableUse'
require_text "$tests" 'TypeCheckDiagnosticKind::TypeMismatch'

echo "m0019-region-exit-refinement-invalidation: test contract validated"
