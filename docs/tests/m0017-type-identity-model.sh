#!/usr/bin/env sh
set -eu

fail() {
  echo "m0017-type-identity-model: $*" >&2
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

task=docs/tasks/M0017-001-type-identity-model.md
milestone=docs/milestones/M0017-type-representation.md
adr_type=docs/adr/ADR-0010-type-system-shape.md
adr_null=docs/adr/ADR-0006-nullability-and-absence.md
adr_generics=docs/adr/ADR-0016-generics-and-parametric-polymorphism.md
source=crates/newlang/src/types.rs
lib=crates/newlang/src/lib.rs
test_file=crates/newlang/tests/types.rs

require_file "$task"
require_file "$milestone"
require_file "$adr_type"
require_file "$adr_null"
require_file "$adr_generics"
require_file "$source"
require_file "$test_file"

require_text "$task" 'Milestone: `M0017`'
require_text "$task" 'Primitive scalar catalog or numeric semantics\.'
require_text "$milestone" 'Nominal type identity\.'
require_text "$adr_type" 'Nominal user-defined types'
require_text "$adr_null" 'explicit optional values'
require_text "$adr_generics" 'Constrained nominal generics'
require_text "$lib" 'pub mod types;'
require_text "$source" 'pub struct TypeId'
require_text "$source" 'pub struct TypeArena'
require_text "$source" 'pub struct TypeRecord'
require_text "$source" 'pub enum TypeKind'
require_text "$source" 'Nominal'
require_text "$source" 'GenericParameter'
require_text "$source" 'pub struct NominalTypeIdentity'
require_text "$source" 'ModuleName'
require_text "$source" 'PackageNamespace'
require_text "$source" 'AstNodeId'
require_text "$source" 'SymbolId'
require_text "$source" 'pub struct GenericParameterType'
require_text "$test_file" 'type_ids_are_stable_in_insertion_order'
require_text "$test_file" 'nominal_type_identity_includes_module_package_declaration_and_symbol'
require_text "$test_file" 'distinct_packages_produce_distinct_nominal_type_identities'
require_text "$test_file" 'generic_parameter_type_preserves_declaring_node_and_symbol'
require_text "$test_file" 'type_record_preserves_kind_and_id'

require_absent_text "$source" 'infer_type|solve_constraints|ConstraintSolver|OwnershipCapability|Layout|Abi|Hir|Mir|Cranelift|LLVM'
require_absent_text "$source" 'PrimitiveScalar|Int32|Int64|Float32|Float64|Bool|StringType'
require_absent_text "$source" 'NullableType|OptionalType|TypeKind::Nullable'

cargo test -p newlang --test types

echo "m0017-type-identity-model: type identity model validation passed"
