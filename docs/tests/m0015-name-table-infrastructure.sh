#!/usr/bin/env sh
set -eu

fail() {
  echo "m0015-name-table: $*" >&2
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

task=docs/tasks/M0015-002-name-table-infrastructure.md
milestone=docs/milestones/M0015-symbol-interning-and-name-tables.md
symbol_src=crates/compiler/src/symbol.rs
symbol_tests=crates/compiler/tests/symbol.rs

require_file "$task"
require_file "$milestone"
require_file "$symbol_src"
require_file "$symbol_tests"

require_text "$task" 'Milestone: `M0015`'
require_text "$symbol_src" 'pub struct NameTable'
require_text "$symbol_src" 'pub struct NameTableKey'
require_text "$symbol_src" 'pub struct NameTableEntry'
require_text "$symbol_src" 'pub enum NameTableInsert'
require_text "$symbol_src" 'pub fn insert'
require_text "$symbol_src" 'pub fn get'
require_text "$symbol_src" 'pub fn entries'
require_text "$symbol_src" 'ModuleName'
require_text "$symbol_src" 'SymbolId'

require_text "$symbol_tests" 'same_text_can_exist_in_distinct_modules'
require_text "$symbol_tests" 'name_table_lookup_uses_exact_module_and_symbol'
require_text "$symbol_tests" 'duplicate_insert_reports_existing_entry_without_replacing'
require_text "$symbol_tests" 'missing_name_table_key_returns_none'
require_text "$task" 'Status: `complete`'
require_text "$milestone" '\[x\] Symbol identities are stable'
require_text "$milestone" '\[x\] Name tables are tested'
require_text "$milestone" '\[x\] Resolution policy is deferred'

require_absent_text "$symbol_src" 'ImportResolver|VisibilityEnforcement|TypeCheck|Overload|AccessCheck|LookupDiagnostic|ScopeStack|ResolutionPolicy'
require_absent_text crates/compiler/src/parser.rs 'NameTable|NameTableKey|NameTableEntry|symbol::'

echo "m0015-name-table: name table infrastructure validation passed"
