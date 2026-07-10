#!/usr/bin/env sh
set -eu

fail() {
  echo "m0015-symbol-interner: $*" >&2
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

task=docs/tasks/M0015-001-symbol-interner.md
milestone=docs/milestones/M0015-symbol-interning-and-name-tables.md
symbol_src=crates/newlang/src/symbol.rs
symbol_tests=crates/newlang/tests/symbol.rs
lib=crates/newlang/src/lib.rs

require_file "$task"
require_file "$milestone"
require_file "$symbol_src"
require_file "$symbol_tests"
require_file "$lib"

require_text "$task" 'Milestone: `M0015`'
require_text "$task" 'Status: `complete`'
require_text "$milestone" '\[x\] Symbol identities are stable'
require_text "$milestone" '\[x\] Name tables are tested'
require_text "$milestone" '\[x\] Resolution policy is deferred'

require_text "$lib" '^pub mod symbol;$'
require_text "$symbol_src" 'pub struct SymbolId'
require_text "$symbol_src" 'pub struct SymbolInterner'
require_text "$symbol_src" 'pub fn intern'
require_text "$symbol_src" 'pub fn resolve'
require_text "$symbol_src" 'pub fn symbols'

require_text "$symbol_tests" 'same_text_interns_to_same_symbol_id'
require_text "$symbol_tests" 'different_text_gets_distinct_stable_ids'
require_text "$symbol_tests" 'unknown_symbol_ids_do_not_resolve'
require_text "$symbol_tests" 'symbols_preserve_exact_text_and_insertion_order'

require_absent_text "$symbol_src" 'Scope|Import|Visibility|TypeCheck|Overload|Resolution|ModuleMetadata|PackageNamespace'
require_absent_text crates/newlang/src/parser.rs 'SymbolInterner|SymbolId|symbol::'

echo "m0015-symbol-interner: symbol interner validation passed"
