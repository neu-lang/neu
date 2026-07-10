#!/usr/bin/env sh
set -eu

fail() {
  echo "m0017-unsupported-type-form-blocking: $*" >&2
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

task=docs/tasks/M0017-003-unsupported-type-form-blocking.md
milestone=docs/milestones/M0017-type-representation.md
adr_diag=docs/adr/ADR-0015-diagnostics-as-semantics.md
adr_syntax=docs/adr/ADR-0023-type-and-generic-syntax.md
spec=docs/SPEC.md
source=crates/newlang/src/types.rs
test_file=crates/newlang/tests/types.rs

require_file "$task"
require_file "$milestone"
require_file "$adr_diag"
require_file "$adr_syntax"
require_file "$spec"
require_file "$source"
require_file "$test_file"

require_text "$task" 'Milestone: `M0017`'
require_text "$task" 'Do not add `TypeKind::Unsupported`'
require_text "$milestone" '- \[x\] Unsupported type forms are blocked\.'
require_text "$spec" 'variance, wildcard types, receiver function types, type aliases'
require_text "$adr_syntax" 'The bootstrap type grammar defers:'
require_text "$adr_diag" 'diagnostic obligations'
require_text "$source" 'pub enum UnsupportedTypeForm'
require_text "$source" 'VarianceAnnotation'
require_text "$source" 'WildcardOrStarProjection'
require_text "$source" 'ReceiverFunctionType'
require_text "$source" 'FunctionTypeParameterName'
require_text "$source" 'TypeAnnotationSyntax'
require_text "$source" 'TypeAlias'
require_text "$source" 'AssociatedType'
require_text "$source" 'HigherKindedType'
require_text "$source" 'DependentType'
require_text "$source" 'IntersectionType'
require_text "$source" 'UnionType'
require_text "$source" 'InferredPlaceholderType'
require_text "$source" 'LayoutType'
require_text "$source" 'EffectType'
require_text "$source" 'CoroutineSuspensionMarker'
require_text "$source" 'pub enum TypeDiagnosticKind'
require_text "$source" 'pub struct TypeDiagnostic'
require_text "$source" 'pub fn unsupported_type_form'
require_text "$test_file" 'unsupported_type_form_diagnostic_preserves_form_and_node'
require_text "$test_file" 'unsupported_type_form_variants_cover_adr0023_deferrals'
require_text "$test_file" 'unsupported_type_forms_do_not_become_type_records'

require_absent_text "$source" 'TypeKind::Unsupported|Unsupported\(UnsupportedTypeForm\)'
require_absent_text "$source" 'lower_type|type_lower|infer_type|solve_constraints|ConstraintSolver|PrimitiveScalar|OwnershipCapability|Cranelift|LLVM'

cargo test -p newlang --test types

echo "m0017-unsupported-type-form-blocking: unsupported type-form blocking validation passed"
