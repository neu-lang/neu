#!/usr/bin/env sh
set -eu

fail() {
  echo "m0014-visibility: $*" >&2
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

adr=docs/adr/ADR-0025-module-package-visibility-model.md
task=docs/tasks/M0014-008-visibility-metadata.md
module_src=crates/compiler/src/module.rs
module_tests=crates/compiler/tests/module.rs
milestone=docs/milestones/M0014-module-package-and-visibility-model.md

require_file "$adr"
require_file "$task"
require_file "$module_src"
require_file "$module_tests"
require_file "$milestone"

require_text "$adr" '^Status: Accepted$'
require_text "$adr" 'Default visibility is `internal`'
require_text "$adr" 'Each declaration has exactly one effective visibility category'
require_text "$adr" 'marks it explicit'
require_text "$adr" 'marks it defaulted'

require_text "$module_src" 'pub enum VisibilityCategory'
require_text "$module_src" 'Public'
require_text "$module_src" 'Internal'
require_text "$module_src" 'Private'
require_text "$module_src" 'pub enum VisibilityOrigin'
require_text "$module_src" 'Explicit'
require_text "$module_src" 'Defaulted'
require_text "$module_src" 'pub struct DeclarationVisibility'
require_text "$module_src" 'pub fn explicit'
require_text "$module_src" 'pub fn default_internal'
require_text "$module_src" 'pub fn visibility'
require_text "$module_src" 'UnsupportedVisibilityCategory'
require_text "$module_src" 'DuplicateVisibilityMetadata'
require_text "$module_src" 'AstNodeId'

require_text "$module_tests" 'visibility_metadata_represents_explicit_categories'
require_text "$module_tests" 'default_visibility_is_internal_and_defaulted'
require_text "$module_tests" 'module_metadata_preserves_declaration_visibility_records'
require_text "$module_tests" 'visibility_metadata_does_not_attach_to_package_or_import_nodes'

require_text "$task" 'Status: `complete`'
require_text "$milestone" '\[x\] Visibility metadata is represented'

require_absent_text "$module_src" 'Protected|Friend|SealedScope|ExtensionScope|ProtocolConformance|AccessCheck|NameResolution|DependencyLookup'
require_absent_text crates/compiler/src/parser.rs 'DeclarationVisibility|VisibilityCategory|VisibilityOrigin|module::'

echo "m0014-visibility: visibility metadata validation passed"
