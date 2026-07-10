#!/usr/bin/env sh
set -eu

fail() {
  echo "m0014-module-identity: $*" >&2
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
task=docs/tasks/M0014-006-module-identity-model.md
module_src=crates/newlang/src/module.rs
lib=crates/newlang/src/lib.rs
module_tests=crates/newlang/tests/module.rs
milestone=docs/milestones/M0014-module-package-and-visibility-model.md

require_file "$adr"
require_file "$task"
require_file "$module_src"
require_file "$lib"
require_file "$module_tests"
require_file "$milestone"

require_text "$adr" '^Status: Accepted$'
require_text "$adr" 'module-name = identifier \(`\.` identifier\)\*'
require_text "$adr" 'The deterministic module ID for tests is the exact module name string after lexical validation'
require_text "$adr" 'Host paths are not module identity'

require_text "$lib" '^pub mod module;$'
require_text "$module_src" 'pub struct ModuleName'
require_text "$module_src" 'pub struct ModuleMetadata'
require_text "$module_src" 'pub enum ModuleDiagnosticKind'
require_text "$module_src" 'MissingModuleIdentity'
require_text "$module_src" 'InvalidModuleIdentity'
require_text "$module_src" 'AmbiguousSourceModuleAssignment'
require_text "$module_src" 'pub fn parse'
require_text "$module_src" 'pub fn deterministic_id'
require_text "$module_src" 'pub fn source_files'
require_text "$module_src" 'SourceFileId'

require_text "$module_tests" 'module_names_validate_adr0025_identifier_segments'
require_text "$module_tests" 'module_name_diagnostics_distinguish_missing_from_invalid'
require_text "$module_tests" 'module_metadata_preserves_explicit_name_and_ordered_source_files'
require_text "$module_tests" 'module_identity_does_not_depend_on_source_file_paths'

require_text "$task" 'Status: `complete`'
require_text "$milestone" '\[x\] Module identity exists'
require_text "$milestone" '\[x\] Visibility metadata is represented'

require_absent_text "$module_src" 'Manifest|TargetTriple|Dependency|PackageManager|NameResolution|SymbolTable|SourceRoot|OutputPath|CurrentDirectory'
require_absent_text "$module_src" 'std::path|PathBuf|Path'
require_absent_text crates/newlang/src/parser.rs 'ModuleName|ModuleMetadata|module::'

echo "m0014-module-identity: module identity model validation passed"
