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
require_text "$source" 'pub struct TopLevelLookup'
require_text "$source" 'pub enum TopLevelLookupResult'
require_text "$source" 'lookup_top_level'
require_text "$source" 'pub struct LocalScopeId'
require_text "$source" 'pub struct LocalScope'
require_text "$source" 'pub struct LocalScopeTree'
require_text "$source" 'pub fn build_local_scope_tree'
require_text "$source" 'pub enum LocalBindingKind'
require_text "$source" 'pub struct LocalBindingKey'
require_text "$source" 'pub struct LocalBinding'
require_text "$source" 'pub enum LocalBindingInsert'
require_text "$source" 'pub struct LocalBindingIndex'
require_text "$source" 'pub struct LocalBindingIndexBuild'
require_text "$source" 'pub fn build_local_binding_index'
require_text "$source" 'pub fn build_scoped_local_binding_index'
require_text "$source" 'pub struct LocalNameLookup'
require_text "$source" 'pub enum LocalNameLookupResult'
require_text "$source" 'lookup_local'
require_text "$source" 'pub struct LocalReferenceBind'
require_text "$source" 'pub fn bind_local_name_references'
require_text "$source" 'Val'
require_text "$source" 'Var'
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
require_text "$test_file" 'top_level_lookup_finds_exact_declaration_key'
require_text "$test_file" 'top_level_lookup_requires_exact_package_and_kind'
require_text "$test_file" 'missing_top_level_lookup_returns_unresolved_name_diagnostic'
require_text "$test_file" 'local_binding_key_preserves_scope_and_symbol'
require_text "$test_file" 'local_binding_index_preserves_insertion_order_and_lookup_by_key'
require_text "$test_file" 'local_binding_index_allows_same_name_in_distinct_scopes'
require_text "$test_file" 'duplicate_local_binding_key_preserves_existing_binding'
require_text "$test_file" 'builds_local_binding_index_from_parser_metadata'
require_text "$test_file" 'local_binding_index_builder_reports_same_scope_duplicates'
require_text "$test_file" 'local_scope_tree_allocates_stable_ids_in_insertion_order'
require_text "$test_file" 'local_scope_tree_preserves_owner_and_parent'
require_text "$test_file" 'local_scope_tree_unknown_scope_id_returns_none'
require_text "$test_file" 'builds_local_scope_tree_for_parser_blocks_in_source_order'
require_text "$test_file" 'local_scope_tree_builder_keeps_declaration_bodies_as_roots'
require_text "$test_file" 'local_scope_tree_builder_ignores_non_scope_owner_nodes'
require_text "$test_file" 'scoped_local_binding_builder_assigns_nearest_block_scope'
require_text "$test_file" 'scoped_local_binding_builder_allows_nested_shadowing'
require_text "$test_file" 'scoped_local_binding_builder_reports_same_block_duplicates'
require_text "$test_file" 'local_binding_lookup_finds_visible_binding_after_declaration'
require_text "$test_file" 'local_binding_lookup_rejects_reference_before_declaration'
require_text "$test_file" 'local_binding_lookup_uses_nearest_visible_scope'
require_text "$test_file" 'local_binding_lookup_continues_past_not_yet_visible_inner_binding'
require_text "$test_file" 'missing_local_binding_lookup_returns_unresolved_name_diagnostic'
require_text "$test_file" 'local_reference_binding_records_visible_local_resolution'
require_text "$test_file" 'local_reference_binding_reports_reference_before_declaration'
require_text "$test_file" 'local_reference_binding_does_not_use_top_level_fallback'
require_text "$test_file" 'diagnostic_kinds_cover_accepted_adr0026_variants'
require_text "$parser" 'pub struct ParsedDeclarationName'
require_text "$parser" 'pub struct ParsedLocalBindingName'
require_text "$parser" 'pub struct ParsedNameReference'
require_text "$parser" 'pub declaration_names: Vec<ParsedDeclarationName>'
require_text "$parser" 'pub local_binding_names: Vec<ParsedLocalBindingName>'
require_text "$parser" 'pub name_references: Vec<ParsedNameReference>'
require_text "$parser_test" 'records_top_level_function_declaration_name_metadata'
require_text "$parser_test" 'records_top_level_type_declaration_name_metadata'
require_text "$parser_test" 'declaration_name_metadata_excludes_nested_declarations_and_missing_names'
require_text "$parser_test" 'records_local_val_and_var_binding_name_metadata'
require_text "$parser_test" 'local_binding_name_metadata_excludes_malformed_declarations'
require_text "$parser_test" 'records_simple_identifier_expression_name_references'
require_text "$parser_test" 'name_reference_metadata_excludes_member_import_and_package_names'

require_absent_text "$source" 'LookupScope|ScopeStack|ImportResolver|VisibilityEnforcement|resolve_names|resolve_module|resolve_file'
require_absent_text "$parser" 'DeclarationIndex|LocalBindingIndex|LocalScopeId|resolve_names|resolve_module|resolve_file|collect_declarations'

echo "m0016-data-model: name resolution data model validation passed"
