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
parser=crates/newlang/src/parser.rs
parser_test=crates/newlang/tests/parser.rs

require_file "$task"
require_file "$adr"
require_file "$source"
require_file "$test_file"
require_file "$parser"
require_file "$parser_test"

require_text "$task" 'Status: `complete`'
require_text "$adr" '^Status: Accepted$'
require_text "$source" 'pub struct ResolvedName'
require_text "$source" 'pub struct ResolutionTable'
require_text "$source" 'pub enum ResolutionInsert'
require_text "$source" 'pub enum DeclarationKind'
require_text "$source" 'pub struct DeclarationKey'
require_text "$source" 'pub struct DeclaredName'
require_text "$source" 'pub enum DeclarationInsert'
require_text "$source" 'pub struct DeclarationIndex'
require_text "$source" 'pub struct DeclarationIndexBuild'
require_text "$source" 'pub fn build_declaration_index'
require_text "$source" 'diagnostics: Vec<ResolutionDiagnostic>'
require_text "$source" 'pub fn diagnostics'
require_text "$source" 'Function'
require_text "$source" 'Type'
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
require_text "$test_file" 'declaration_key_preserves_adr0026_top_level_tuple'
require_text "$test_file" 'declaration_index_key_includes_module_package_and_kind'
require_text "$test_file" 'duplicate_declaration_key_preserves_existing_declaration'
require_text "$test_file" 'builds_declaration_index_from_parser_metadata_and_module_package'
require_text "$test_file" 'declaration_index_builder_preserves_duplicate_insert_results'
require_text "$test_file" 'declaration_index_builder_keeps_same_name_in_distinct_packages'
require_text "$test_file" 'duplicate_declaration_diagnostics_do_not_replace_existing_declaration'
require_text "$test_file" 'diagnostic_kinds_cover_accepted_adr0026_variants'
require_text "$parser" 'pub struct ParsedDeclarationName'
require_text "$parser" 'pub declaration_names: Vec<ParsedDeclarationName>'
require_text "$parser_test" 'records_top_level_function_declaration_name_metadata'
require_text "$parser_test" 'records_top_level_type_declaration_name_metadata'
require_text "$parser_test" 'declaration_name_metadata_excludes_nested_declarations_and_missing_names'

require_absent_text "$source" 'LookupScope|ScopeStack|ImportResolver|VisibilityEnforcement|resolve_names|resolve_module|resolve_file'
require_absent_text "$parser" 'DeclarationIndex|resolve_names|resolve_module|resolve_file|collect_declarations'

echo "m0016-data-model: name resolution data model validation passed"
