#!/usr/bin/env sh
set -eu

fail() {
  echo "m0019-flow-data-model: $*" >&2
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
task=docs/tasks/M0019-006-flow-output-data-model.md
adr=docs/adr/ADR-0028-nullability-and-flow-typing.md

require_file "$source"
require_file "$tests"
require_file "$task"
require_file "$adr"

require_text "$adr" '^Status: Accepted$'
require_text "$source" 'pub struct RefinementRecord'
require_text "$source" 'pub struct RefinedExpressionType'
require_text "$source" 'InvalidNullableUse'
require_text "$source" 'InvalidatedRefinement'
require_text "$source" 'UnsupportedFlowRule'
require_text "$source" 'AmbiguousFlowRule'
require_text "$source" 'NullableValueWithoutRefinement'
require_text "$source" 'NullableAssignmentWithoutRefinement'
require_text "$source" 'AssignmentInvalidatedRefinement'
require_text "$source" 'RegionExitInvalidatedRefinement'
require_text "$source" 'MutableLocalRefinementDeferred'
require_text "$source" 'BooleanCombinationRefinementDeferred'
require_text "$source" 'MemberRefinementDeferred'
require_text "$source" 'CallResultRefinementDeferred'
require_text "$source" 'ExclusiveBorrowRefinementDeferred'
require_text "$source" 'AmbiguousLocalBindingFlow'
require_text "$source" 'AmbiguousNullTestRegion'
require_text "$source" 'record_refinement'
require_text "$source" 'record_refined_expression_type'
require_text "$source" 'original_nullable_type'
require_text "$source" 'refined_non_null_type'
require_text "$tests" 'm0019_flow_diagnostic_constructors_preserve_rule_node_and_types'
require_text "$tests" 'm0019_type_check_report_records_flow_refinements_in_insertion_order'

require_absent_text "$source" 'recognize_null_test|apply_smart_cast|check_nullable_use|walk_if_branch'
require_absent_text crates/compiler/src/lib.rs 'pub mod hir|pub mod mir|pub mod backend'

echo "m0019-flow-data-model: flow output data model validation passed"
