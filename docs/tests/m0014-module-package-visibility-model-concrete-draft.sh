#!/usr/bin/env sh
set -eu

fail() {
  echo "m0014-concrete-draft: $*" >&2
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

proposal=docs/adr/proposals/ADR-0025-module-package-visibility-model.md
ambiguity=docs/ambiguities/M0014-module-package-visibility-model.md
decision=docs/adr/proposals/reviews/ADR-0025-chief-architect-decision.md
task=docs/tasks/M0014-004-module-package-visibility-concrete-draft.md

require_file "$proposal"
require_file "$ambiguity"
require_file "$decision"
require_file "$task"

require_text "$proposal" '^Status: Draft proposal - not accepted source of truth$'
require_text "$proposal" '^## Concrete Draft Model$'
require_text "$proposal" '^### Module Identity$'
require_text "$proposal" '^### Source File Assignment$'
require_text "$proposal" '^### Package Namespace Model$'
require_text "$proposal" '^### Visibility Categories$'
require_text "$proposal" '^### Visibility Metadata$'
require_text "$proposal" '^### Module Metadata Record$'
require_text "$proposal" '^### Required Diagnostics$'
require_text "$proposal" '^### Explicit Draft Deferrals$'
require_text "$proposal" 'module-name = identifier \(`\.` identifier\)\*'
require_text "$proposal" 'default visibility is `internal`'
require_text "$proposal" '`private` means visible only within the declaring source file'
require_text "$proposal" '`internal` means visible within the same module'
require_text "$proposal" 'root package'
require_text "$proposal" 'host paths are not module identity'
require_text "$proposal" '\| Diagnostic \| Primary span or external input location \| Recovery action \| Safe suggestion \|'
require_text "$proposal" 'missing_module_identity'
require_text "$proposal" 'invalid_module_identity'
require_text "$proposal" 'ambiguous_source_module_assignment'
require_text "$proposal" 'invalid_package_namespace'
require_text "$proposal" 'unsupported_visibility_category'
require_text "$proposal" 'duplicate_visibility_metadata'

require_text "$ambiguity" 'Status: `open`'
require_text "$decision" '^Decision: pending-revision$'
require_text "$task" 'Status: `complete`'

require_absent_text docs/SPEC.md '^## ADR-0025: Module Package And Visibility Model$'
require_absent_text crates/newlang/src/lib.rs 'pub mod module|pub mod modules|pub mod name_resolution'

echo "m0014-concrete-draft: module package and visibility concrete draft validation passed"
