#!/usr/bin/env sh
set -eu

fail() {
  echo "m0014-package-namespace: $*" >&2
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
task=docs/tasks/M0014-007-package-namespace-metadata.md
module_src=crates/compiler/src/module.rs
module_tests=crates/compiler/tests/module.rs
milestone=docs/milestones/M0014-module-package-and-visibility-model.md

require_file "$adr"
require_file "$task"
require_file "$module_src"
require_file "$module_tests"
require_file "$milestone"

require_text "$adr" '^Status: Accepted$'
require_text "$adr" 'Packages are namespaces inside a module'
require_text "$adr" 'The root package is represented as the empty package path'
require_text "$adr" 'package namespace for each source file'

require_text "$module_src" 'pub struct PackageNamespace'
require_text "$module_src" 'pub struct SourceFilePackage'
require_text "$module_src" 'InvalidPackageNamespace'
require_text "$module_src" 'pub fn root'
require_text "$module_src" 'pub fn is_root'
require_text "$module_src" 'pub fn packages'
require_text "$module_src" 'pub fn with_packages'

require_text "$module_tests" 'package_namespaces_validate_adr0025_segments_and_root'
require_text "$module_tests" 'invalid_package_namespaces_report_package_diagnostics'
require_text "$module_tests" 'module_metadata_preserves_package_namespace_per_source_file'
require_text "$module_tests" 'package_namespace_does_not_change_module_identity'

require_text "$task" 'Status: `complete`'
require_text "$milestone" '\[x\] Package namespace metadata is represented'
require_text "$milestone" '\[x\] Visibility metadata is represented'

require_absent_text "$module_src" 'PackageManager|Manifest|TargetTriple|Dependency|NameResolution|SymbolTable|ImportResolver'
require_absent_text crates/compiler/src/parser.rs 'PackageNamespace|SourceFilePackage|module::'

echo "m0014-package-namespace: package namespace metadata validation passed"
