#!/usr/bin/env sh
set -eu

fail() {
  echo "m0019-refined-expression-type-records: $*" >&2
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

source=crates/newlang/src/type_check.rs
tests=crates/newlang/tests/type_check.rs
task=docs/tasks/M0019-012-refined-expression-type-records.md

require_file "$source"
require_file "$tests"
require_file "$task"

require_text "$source" 'pub fn record_m0019_refined_expression_types'
require_text "$source" 'ResolvedLocalBinding'
require_text "$source" 'AstNodeKind::NameExpression'
require_text "$source" 'AstNodeKind::Block'
require_text "$source" 'record_refined_expression_type'
require_text "$source" 'TypeRuleDiagnostic::AmbiguousNullTestRegion'
require_text "$tests" 'm0019_refined_expression_type_records_active_exact_binding_uses'
require_text "$tests" 'm0019_refined_expression_type_records_honor_nested_shadowing_and_region_bounds'
require_text "$tests" 'm0019_refined_expression_type_records_report_overlapping_regions'
require_text "$tests" 'm0019_refined_expression_type_records_reject_non_name_and_cross_file_uses'

echo "m0019-refined-expression-type-records: per-use refinement validation passed"
