#!/usr/bin/env sh
set -eu

fail() {
  echo "m0017-nullable-type-representation: $*" >&2
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

task=docs/tasks/M0017-002-nullable-type-representation.md
milestone=docs/milestones/M0017-type-representation.md
adr_null=docs/adr/ADR-0006-nullability-and-absence.md
adr_syntax=docs/adr/ADR-0023-type-and-generic-syntax.md
source=crates/newlang/src/types.rs
test_file=crates/newlang/tests/types.rs

require_file "$task"
require_file "$milestone"
require_file "$adr_null"
require_file "$adr_syntax"
require_file "$source"
require_file "$test_file"

require_text "$task" 'Milestone: `M0017`'
require_text "$task" 'Smart casts and flow typing\.'
require_text "$milestone" '- \[x\] Type identity is represented\.'
require_text "$milestone" '- \[x\] Nullable types are represented\.'
require_text "$adr_null" 'explicit optional values'
require_text "$adr_syntax" 'nullable-type = primary-type `\?`\?'
require_text "$source" 'pub struct NullableType'
require_text "$source" 'base: TypeId'
require_text "$source" 'pub fn base'
require_text "$source" 'Nullable\(NullableType\)'
require_text "$source" 'pub fn nullable'
require_text "$test_file" 'nullable_type_preserves_wrapped_base_type'
require_text "$test_file" 'nullable_type_record_is_distinct_from_base_record'
require_text "$test_file" 'nullable_record_storage_preserves_insertion_order'

require_absent_text "$source" 'smart_cast|flow_typ|FlowType|MutationInvalidation|NullCheck|NullLiteral|FfiNull|PlatformNull'
require_absent_text "$source" 'infer_type|solve_constraints|ConstraintSolver|OwnershipCapability|Cranelift|LLVM'

cargo test -p newlang --test types

echo "m0017-nullable-type-representation: nullable type representation validation passed"
