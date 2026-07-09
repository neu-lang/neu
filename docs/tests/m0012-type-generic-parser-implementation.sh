#!/usr/bin/env sh
set -eu

fail() {
  echo "m0012-parser-impl: $*" >&2
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

parser=crates/newlang/src/parser.rs
parser_tests=crates/newlang/tests/parser.rs
task=docs/tasks/M0012-008-type-generic-parser-implementation.md
milestone=docs/milestones/M0012-type-and-generic-syntax-parser.md

require_file "$parser"
require_file "$parser_tests"
require_file "$task"
require_file "$milestone"

require_text "$parser" 'fn parse_type'
require_text "$parser" 'fn parse_named_type'
require_text "$parser" 'fn parse_generic_parameters'
require_text "$parser" 'fn parse_generic_arguments'
require_text "$parser" 'fn parse_capability_bound'
require_text "$parser" 'fn parse_function_type'

require_text "$parser" 'MissingTypeName'
require_text "$parser" 'MalformedNullableType'
require_text "$parser" 'MalformedGenericParameterList'
require_text "$parser" 'MalformedGenericArgumentList'
require_text "$parser" 'MissingGenericBound'
require_text "$parser" 'MalformedCapabilityBound'
require_text "$parser" 'MalformedFunctionType'
require_text "$parser" 'UnsupportedTypeForm'
require_text "$parser" 'UnexpectedTokenInType'

require_text "$parser_tests" 'parses_type_and_generic_syntax'
require_text "$parser_tests" 'reports_malformed_type_and_generic_syntax'
require_text "$parser_tests" 'AstNodeKind::NamedType'
require_text "$parser_tests" 'AstNodeKind::NullableType'
require_text "$parser_tests" 'AstNodeKind::GenericParameter'
require_text "$parser_tests" 'AstNodeKind::GenericArgument'
require_text "$parser_tests" 'AstNodeKind::CapabilityBound'
require_text "$parser_tests" 'AstNodeKind::FunctionType'
require_text "$parser_tests" 'AstNodeKind::GroupedType'

require_text "$task" 'Status: `complete`'
require_text "$milestone" '\[x\] Approved type syntax parses'
require_text "$milestone" '\[x\] Invalid type syntax reports source-spanned diagnostics'

require_absent_text "$parser" 'type_check|constraint_solving|resolve_type|CapabilitySet|Hir|Mir|parse_expression|parse_statement|parse_pattern|parse_coroutine|parse_unsafe'

echo "m0012-parser-impl: type and generic parser implementation validation passed"
