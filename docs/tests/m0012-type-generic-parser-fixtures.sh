#!/usr/bin/env sh
set -eu

fail() {
  echo "m0012-fixtures: $*" >&2
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
task=docs/tasks/M0012-006-type-generic-parser-fixtures.md

require_file "$adr"
require_file "$task"
require_text "$adr" '^Status: Accepted$'
require_text "$task" 'Milestone: `M0012`'

for fixture in \
  tests/fixtures/parser/types/positive.fixture.toml \
  tests/fixtures/parser/types/negative.fixture.toml \
  tests/fixtures/parser/types/diagnostics.fixture.toml \
  tests/fixtures/parser/generics/positive.fixture.toml \
  tests/fixtures/parser/generics/negative.fixture.toml \
  tests/fixtures/parser/generics/diagnostics.fixture.toml
do
  require_file "$fixture"
  require_text "$fixture" '^kind = "parser-type-generic-fixture"$'
  require_text "$fixture" '^milestone = "M0012"$'
  require_text "$fixture" '^source = "docs/adr/ADR-0023-type-and-generic-syntax.md"$'
  require_absent_text "$fixture" 'Kotlin|Rust|Go'
  require_absent_text "$fixture" 'type_check|constraint_solving|capability_semantics|hir|mir|backend'
done

require_text tests/fixtures/parser/types/positive.fixture.toml 'name = "named_and_qualified_type_refs"'
require_text tests/fixtures/parser/types/positive.fixture.toml 'source_text = "Int demo.core.Box"'
require_text tests/fixtures/parser/types/positive.fixture.toml 'name = "nullable_and_generic_type_binding"'
require_text tests/fixtures/parser/types/positive.fixture.toml 'source_text = "Box<T\?>\?"'
require_text tests/fixtures/parser/types/positive.fixture.toml 'name = "function_type_binding"'
require_text tests/fixtures/parser/types/positive.fixture.toml 'source_text = "\(T\) -> U\? \(\(T\) -> U\)\?"'

require_text tests/fixtures/parser/generics/positive.fixture.toml 'name = "generic_parameters_with_capability_bounds"'
require_text tests/fixtures/parser/generics/positive.fixture.toml 'source_text = "<T: Send & Share, U>"'
require_text tests/fixtures/parser/generics/positive.fixture.toml 'expected_bounds = \["Send", "Share"\]'
require_text tests/fixtures/parser/generics/positive.fixture.toml 'name = "generic_arguments_nested_types"'
require_text tests/fixtures/parser/generics/positive.fixture.toml 'source_text = "Map<String, Box<T\?>>"'

require_text tests/fixtures/parser/types/negative.fixture.toml 'name = "repeated_nullable_marker_is_malformed"'
require_text tests/fixtures/parser/types/negative.fixture.toml 'malformed_nullable_type'
require_text tests/fixtures/parser/types/negative.fixture.toml 'name = "unsupported_type_forms_are_rejected"'
require_text tests/fixtures/parser/types/negative.fixture.toml 'unsupported_type_form'

require_text tests/fixtures/parser/generics/negative.fixture.toml 'name = "empty_generic_parameter_list_is_malformed"'
require_text tests/fixtures/parser/generics/negative.fixture.toml 'malformed_generic_parameter_list'
require_text tests/fixtures/parser/generics/negative.fixture.toml 'name = "comma_separated_bounds_are_malformed"'
require_text tests/fixtures/parser/generics/negative.fixture.toml 'malformed_capability_bound'

require_text tests/fixtures/parser/types/diagnostics.fixture.toml 'missing_type_name'
require_text tests/fixtures/parser/types/diagnostics.fixture.toml 'malformed_nullable_type'
require_text tests/fixtures/parser/types/diagnostics.fixture.toml 'malformed_function_type'
require_text tests/fixtures/parser/types/diagnostics.fixture.toml 'unsupported_type_form'
require_text tests/fixtures/parser/types/diagnostics.fixture.toml 'unexpected_token_in_type'
require_text tests/fixtures/parser/types/diagnostics.fixture.toml 'type recovery boundary'

require_text tests/fixtures/parser/generics/diagnostics.fixture.toml 'malformed_generic_parameter_list'
require_text tests/fixtures/parser/generics/diagnostics.fixture.toml 'malformed_generic_argument_list'
require_text tests/fixtures/parser/generics/diagnostics.fixture.toml 'missing_generic_bound'
require_text tests/fixtures/parser/generics/diagnostics.fixture.toml 'malformed_capability_bound'
require_text tests/fixtures/parser/generics/diagnostics.fixture.toml 'generic parameter boundary'

require_absent_text crates/newlang/src/parser.rs 'parse_type|parse_generic|parse_capability|TypeRef|GenericParameter|GenericArgument|CapabilityBound|NullableType|FunctionType'
require_absent_text crates/newlang/src/ast.rs 'TypeRef|GenericParameter|GenericArgument|CapabilityBound|NullableType|FunctionType'

echo "m0012-fixtures: type and generic parser fixture validation passed"
