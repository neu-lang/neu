#!/usr/bin/env sh
set -eu

fail() {
  echo "m0014-blocked: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope path exists while M0014 is blocked: $1"
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

task=docs/tasks/M0014-001-module-package-visibility-model-blocker.md
ambiguity=docs/ambiguities/M0014-module-package-visibility-model.md
milestone=docs/milestones/M0014-module-package-and-visibility-model.md
adr=docs/adr/ADR-0017-modules-visibility-and-api-evolution.md
spec=docs/SPEC.md

require_file "$task"
require_file "$ambiguity"
require_file "$milestone"
require_file "$adr"
require_file "$spec"

require_text "$task" '^# Task: M0014-001 Record module package and visibility model blocker$'
require_text "$task" 'Status: `complete`'
require_text "$task" 'Milestone: `M0014`'
require_text "$task" 'Language Designer to draft a non-authoritative module, package, namespace, and visibility model proposal'

require_text "$ambiguity" '^# Ambiguity Report: M0014 Module, Package, And Visibility Model$'
require_text "$ambiguity" 'Status: `open`'
require_text "$ambiguity" 'Blocking milestone: `M0014`'
require_text "$ambiguity" 'module identity'
require_text "$ambiguity" 'package-to-module mapping'
require_text "$ambiguity" 'default visibility'
require_text "$ambiguity" '`internal` meaning'
require_text "$ambiguity" 'No implementation may define module identity'

require_text "$adr" 'Modules as explicit compilation and visibility units'
require_text "$adr" 'Public/private/internal visibility must be specified'
require_text "$spec" '^## ADR-0017: Modules, Visibility, And API Evolution$'
require_text "$spec" 'Modules are explicit compilation and visibility units'

require_text "$milestone" 'M0014'
require_text "$milestone" '\[ \] Module identity exists'
require_text "$milestone" '\[ \] Visibility metadata is represented'
require_text "$milestone" '\[x\] Unspecified visibility rules are recorded'

require_absent_path crates/newlang/src/module.rs
require_absent_path crates/newlang/src/modules.rs
require_absent_path crates/newlang/src/name_resolution.rs
require_absent_text crates/newlang/src/lib.rs 'pub mod module|pub mod modules|pub mod name_resolution'
require_absent_text crates/newlang/src/parser.rs 'ModuleId|PackageId|VisibilityModel|DefaultVisibility|InternalVisibility|ModuleDependency'

echo "m0014-blocked: module package and visibility model ambiguity validation passed"
