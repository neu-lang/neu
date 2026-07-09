#!/usr/bin/env sh
set -eu

fail() {
  echo "m0012-type-ast: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

require_absent_text() {
  file="$1"
  pattern="$2"
  if grep -Eq "$pattern" "$file"; then
    fail "unexpected pattern in $file: $pattern"
  fi
}

adr=docs/adr/ADR-0023-type-and-generic-syntax.md
task=docs/tasks/M0012-007-type-ast-shell.md
ast=crates/newlang/src/ast.rs
ast_tests=crates/newlang/tests/ast.rs

require_file "$adr"
require_file "$task"
require_file "$ast"
require_file "$ast_tests"

require_text "$adr" '^Status: Accepted$'
require_text "$task" 'Milestone: `M0012`'

for name in \
  NamedType \
  NullableType \
  GenericParameter \
  GenericArgument \
  CapabilityBound \
  FunctionType \
  GroupedType
do
  require_text "$ast" "$name"
done

require_text "$ast" 'add_named_type'
require_text "$ast" 'add_nullable_type'
require_text "$ast" 'add_generic_parameter'
require_text "$ast" 'add_generic_argument'
require_text "$ast" 'add_capability_bound'
require_text "$ast" 'add_function_type'
require_text "$ast" 'add_grouped_type'

require_text "$ast_tests" 'type_and_generic_shell_nodes_preserve_kind_and_span'
require_text "$ast_tests" 'AstNodeKind::NamedType'
require_text "$ast_tests" 'AstNodeKind::NullableType'
require_text "$ast_tests" 'AstNodeKind::GenericParameter'
require_text "$ast_tests" 'AstNodeKind::GenericArgument'
require_text "$ast_tests" 'AstNodeKind::CapabilityBound'
require_text "$ast_tests" 'AstNodeKind::FunctionType'
require_text "$ast_tests" 'AstNodeKind::GroupedType'

require_text "$task" 'Status: `complete`'

require_absent_text "$ast" 'TypeId|ResolvedType|SymbolId|Constraint|CapabilitySet|Borrow|Lifetime|Coroutine|Unsafe'
require_absent_text crates/newlang/src/parser.rs 'parse_type|parse_generic|parse_capability|TypeRef|GenericParameter|GenericArgument|CapabilityBound|NullableType|FunctionType'

echo "m0012-type-ast: type and generic AST shell validation passed"
