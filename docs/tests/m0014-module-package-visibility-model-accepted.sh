#!/usr/bin/env sh
set -eu

fail() {
  echo "m0014-accepted: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope path exists after ADR acceptance: $1"
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
spec=docs/SPEC.md
ambiguity=docs/ambiguities/M0014-module-package-visibility-model.md
decision=docs/adr/proposals/reviews/ADR-0025-chief-architect-decision.md
milestone=docs/milestones/M0014-module-package-and-visibility-model.md
task=docs/tasks/M0014-005-accept-module-package-visibility-model-adr.md

require_file "$adr"
require_file "$spec"
require_file "$ambiguity"
require_file "$decision"
require_file "$milestone"
require_file "$task"

require_text "$adr" '^# ADR-0025: Module Package And Visibility Model$'
require_text "$adr" '^Status: Accepted$'
require_text "$adr" '^## Decision$'
require_text "$adr" 'explicit module name supplied by the compiler invocation or test harness'
require_text "$adr" 'module-name = identifier \(`\.` identifier\)\*'
require_text "$adr" 'Host paths are not module identity'
require_text "$adr" 'Each parsed source file belongs to exactly one module'
require_text "$adr" 'Packages are namespaces inside a module'
require_text "$adr" 'root package is represented as the empty package path'
require_text "$adr" 'Default visibility is `internal`'
require_text "$adr" '`public` means visible to other modules'
require_text "$adr" '`internal` means visible within the same module'
require_text "$adr" '`private` means visible only within the declaring source file'
require_text "$adr" 'Each declaration has exactly one effective visibility category'
require_text "$adr" '\| Diagnostic \| Primary span or external input location \| Recovery action \| Safe suggestion \| Source-of-truth citation \|'
require_text "$adr" 'missing_module_identity'
require_text "$adr" 'invalid_module_identity'
require_text "$adr" 'ambiguous_source_module_assignment'
require_text "$adr" 'invalid_package_namespace'
require_text "$adr" 'unsupported_visibility_category'
require_text "$adr" 'duplicate_visibility_metadata'
require_text "$adr" 'ADR-0015, ADR-0017, ADR-0022, ADR-0025'
require_absent_text "$adr" '^Status: Draft proposal'
require_absent_text "$adr" '^## Non-Authority Notice$'

require_text "$spec" '^## ADR-0025: Module Package And Visibility Model$'
require_text "$spec" 'The bootstrap frontend uses explicit module names supplied by compiler invocation or tests'
require_text "$spec" 'Host paths are not module identity'
require_text "$spec" 'Packages are namespaces inside modules'
require_text "$spec" 'Default visibility is `internal`'
require_text "$spec" 'M0014 module metadata includes module name, ordered source file identities, package namespace per source file, and effective visibility metadata'

require_text "$ambiguity" 'Status: `resolved`'
require_text "$ambiguity" 'Resolution Source: `docs/adr/ADR-0025-module-package-visibility-model.md`'
require_text "$ambiguity" 'Resolved Date: `2026-07-10`'
require_text "$ambiguity" '\[x\] Language Designer drafts'
require_text "$ambiguity" '\[x\] Chief Architect approves final source-of-truth update'
require_text "$ambiguity" 'Implementation may define module identity, package-to-module mapping, default visibility, `internal` meaning, and visibility diagnostics only as specified by accepted ADR-0025'

require_text "$decision" '^Decision: approved$'
require_text "$decision" 'ADR-0025 is accepted'
require_text "$decision" 'M0014 ambiguity is resolved'

require_text "$milestone" '\[x\] Module, package, and visibility source of truth is accepted'
require_text "$task" 'Status: `complete`'

require_absent_path crates/newlang/src/module.rs
require_absent_path crates/newlang/src/modules.rs
require_absent_path crates/newlang/src/name_resolution.rs
require_absent_text crates/newlang/src/lib.rs 'pub mod module|pub mod modules|pub mod name_resolution'
require_absent_text crates/newlang/src/parser.rs 'ModuleId|PackageId|VisibilityModel|DefaultVisibility|InternalVisibility|ModuleDependency'

echo "m0014-accepted: module package and visibility accepted ADR validation passed"
