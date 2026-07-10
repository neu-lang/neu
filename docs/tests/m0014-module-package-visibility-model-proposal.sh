#!/usr/bin/env sh
set -eu

fail() {
  echo "m0014-proposal: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope path exists during proposal task: $1"
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
accepted=docs/adr/ADR-0025-module-package-visibility-model.md
ambiguity=docs/ambiguities/M0014-module-package-visibility-model.md
task=docs/tasks/M0014-002-module-package-visibility-model-proposal.md

require_file "$proposal"
require_file "$accepted"
require_file "$ambiguity"
require_file "$task"

require_text "$proposal" '^# ADR-0025: Module Package And Visibility Model$'
require_text "$proposal" '^Status: Draft proposal - not accepted source of truth$'
require_text "$proposal" '^## Non-Authority Notice$'
require_text "$proposal" '^## Question$'
require_text "$proposal" '^## Competing Designs$'
require_text "$proposal" '^## Trade-offs$'
require_text "$proposal" '^## Recommended Draft Choice$'
require_text "$proposal" '^## Draft Model Direction$'
require_text "$proposal" '^## Required Accepted Content$'
require_text "$proposal" '^## Required Diagnostics$'
require_text "$proposal" '^## Downstream Consequences$'
require_text "$proposal" '^## Dependencies$'
require_text "$proposal" 'No implementation may depend on this proposal until accepted'
require_text "$proposal" 'module identity'
require_text "$proposal" 'package-to-module mapping'
require_text "$proposal" 'visibility categories'
require_text "$proposal" 'public'
require_text "$proposal" 'private'
require_text "$proposal" 'internal'
require_text "$proposal" 'not rely on Kotlin, Rust, Go, file paths, or existing compiler behavior as implicit authority'

require_text "$accepted" '^Status: Accepted$'
require_text "$ambiguity" 'Status: `resolved`'
require_text "$ambiguity" 'Blocking milestone: `M0014`'
require_text "$task" 'Status: `complete`'

require_text docs/SPEC.md '^## ADR-0025: Module Package And Visibility Model$'
require_absent_path crates/newlang/src/modules.rs
require_absent_path crates/newlang/src/name_resolution.rs

echo "m0014-proposal: module package and visibility model proposal validation passed"
