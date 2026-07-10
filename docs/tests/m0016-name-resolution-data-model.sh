#!/usr/bin/env sh
set -eu

fail() {
  echo "m0016-data-model: $*" >&2
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

task=docs/tasks/M0016-006-name-resolution-data-model.md
adr=docs/adr/ADR-0026-name-resolution-policy.md
source=crates/newlang/src/name_resolution.rs
lib=crates/newlang/src/lib.rs
test_file=crates/newlang/tests/name_resolution.rs

require_file "$task"
require_file "$adr"
require_file "$source"
require_file "$test_file"

require_text "$task" 'Status: `complete`'
require_text "$adr" '^Status: Accepted$'
require_text "$source" 'pub struct ResolvedName'
require_text "$source" 'pub struct ResolutionTable'
require_text "$source" 'pub enum ResolutionInsert'
require_text "$source" 'pub enum ResolutionDiagnosticKind'
require_text "$source" 'UnresolvedName'
require_text "$source" 'DuplicateName'
require_text "$source" 'AmbiguousName'
require_text "$source" 'UnsupportedImportResolution'
require_text "$source" 'UnsupportedCrossModuleLookup'
require_text "$source" 'UnsupportedMemberResolution'
require_text "$source" 'pub struct ResolutionDiagnostic'
require_text "$lib" 'pub mod name_resolution;'
require_text "$test_file" 'duplicate_resolved_name_insert_preserves_existing_record'
require_text "$test_file" 'diagnostic_kinds_cover_accepted_adr0026_variants'

require_absent_text "$source" 'LookupScope|ScopeStack|ImportResolver|VisibilityEnforcement|resolve_names|resolve_module|resolve_file'

echo "m0016-data-model: name resolution data model validation passed"
