#!/usr/bin/env sh
set -eu

fail() {
  echo "m0011-fixtures: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope implementation path exists during fixture task: $1"
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

adr=docs/adr/ADR-0022-declaration-syntax.md
task=docs/tasks/M0011-006-declaration-parser-fixtures.md

require_file "$adr"
require_file "$task"
require_text "$adr" '^Status: Accepted$'
require_text "$task" 'Milestone: `M0011`'

for fixture in \
  tests/fixtures/parser/declarations/positive.fixture.toml \
  tests/fixtures/parser/declarations/negative.fixture.toml \
  tests/fixtures/parser/declarations/diagnostics.fixture.toml
do
  require_file "$fixture"
  require_text "$fixture" '^kind = "parser-declaration-fixture"$'
  require_text "$fixture" '^milestone = "M0011"$'
  require_text "$fixture" '^source = "docs/adr/ADR-0022-declaration-syntax.md"$'
  require_absent_text "$fixture" 'Kotlin|Rust|Go'
  require_absent_text "$fixture" 'parser_precedence|hir|mir|backend'
done

require_text tests/fixtures/parser/declarations/positive.fixture.toml 'name = "package_imports_and_function"'
require_text tests/fixtures/parser/declarations/positive.fixture.toml 'source_text = "package demo.core import demo.io as io public fun main\(\);"' 
require_text tests/fixtures/parser/declarations/positive.fixture.toml 'expected_declarations = \["package", "import", "function"\]'
require_text tests/fixtures/parser/declarations/positive.fixture.toml 'name = "nested_declaration_body"'
require_text tests/fixtures/parser/declarations/positive.fixture.toml 'expected_declarations = \["struct", "function", "interface", "enum"\]'

require_text tests/fixtures/parser/declarations/negative.fixture.toml 'name = "field_syntax_is_deferred"'
require_text tests/fixtures/parser/declarations/negative.fixture.toml 'unsupported_declaration_modifier'
require_text tests/fixtures/parser/declarations/negative.fixture.toml 'name = "type_parameters_are_deferred"'
require_text tests/fixtures/parser/declarations/negative.fixture.toml 'malformed_declaration_header'
require_absent_text tests/fixtures/parser/declarations/negative.fixture.toml 'expected_type|expected_expression|expected_statement'

require_text tests/fixtures/parser/declarations/diagnostics.fixture.toml 'misplaced_package_declaration'
require_text tests/fixtures/parser/declarations/diagnostics.fixture.toml 'misplaced_import_declaration'
require_text tests/fixtures/parser/declarations/diagnostics.fixture.toml 'duplicate_visibility_modifier'
require_text tests/fixtures/parser/declarations/diagnostics.fixture.toml 'missing_declaration_name'
require_text tests/fixtures/parser/declarations/diagnostics.fixture.toml 'invalid_member_declaration_position'
require_text tests/fixtures/parser/declarations/diagnostics.fixture.toml 'unexpected_token_in_declaration_body'
require_text tests/fixtures/parser/declarations/diagnostics.fixture.toml 'skip-to-declaration-boundary'

require_absent_path crates/newlang/src/parser.rs
require_absent_path crates/newlang/tests/parser.rs

echo "m0011-fixtures: declaration parser fixture validation passed"
