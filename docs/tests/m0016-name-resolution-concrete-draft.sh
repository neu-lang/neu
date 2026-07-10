#!/usr/bin/env sh
set -eu

fail() {
  echo "m0016-concrete-draft: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope path exists during concrete draft task: $1"
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

proposal=docs/adr/proposals/ADR-0026-name-resolution-policy.md
ambiguity=docs/ambiguities/M0016-name-resolution-policy.md
task=docs/tasks/M0016-004-name-resolution-concrete-draft.md

require_file "$proposal"
require_file "$ambiguity"
require_file "$task"

require_text "$proposal" '^Status: Draft proposal - not accepted source of truth$'
require_text "$proposal" '^## Draft Concrete Resolution Model$'
require_text "$proposal" '^## Draft Resolvable AST Node Kinds$'
require_text "$proposal" '^## Draft Declaration And Binding Positions$'
require_text "$proposal" '^## Draft Scope And Declaration Order$'
require_text "$proposal" '^## Draft Shadowing And Duplicate Rules$'
require_text "$proposal" '^## Draft Lookup Rules$'
require_text "$proposal" '^## Draft Visibility Rule$'
require_text "$proposal" '^## Draft Resolution Diagnostics$'
require_text "$proposal" '^## Draft Unsupported Forms$'

require_text "$proposal" 'simple identifier expression'
require_text "$proposal" 'qualified name expression'
require_text "$proposal" 'type name node'
require_text "$proposal" 'package-qualified name'
require_text "$proposal" 'function declaration name'
require_text "$proposal" 'type declaration name'
require_text "$proposal" 'local `val` statement'
require_text "$proposal" 'local `var` statement'
require_text "$proposal" 'pattern bindings are excluded'
require_text "$proposal" 'function parameters are excluded'
require_text "$proposal" 'local bindings are not visible before their declaration statement'
require_text "$proposal" 'inner local declaration shadows'
require_text "$proposal" 'same-scope duplicate local binding'
require_text "$proposal" 'same-module same-package duplicate top-level declaration'
require_text "$proposal" 'ambiguity instead of choosing by insertion order'
require_text "$proposal" 'imports remain syntax-only'
require_text "$proposal" 'cross-module lookup remains unsupported'
require_text "$proposal" 'member lookup remains unsupported'
require_text "$proposal" 'overload resolution remains unsupported'
require_text "$proposal" 'extension method lookup remains unsupported'
require_text "$proposal" 'type-directed lookup remains unsupported'

require_text "$proposal" 'Diagnostic: `unresolved_name`'
require_text "$proposal" 'Diagnostic: `duplicate_name`'
require_text "$proposal" 'Diagnostic: `ambiguous_name`'
require_text "$proposal" 'Diagnostic: `unsupported_import_resolution`'
require_text "$proposal" 'Primary span:'
require_text "$proposal" 'Recovery action:'
require_text "$proposal" 'Source-of-truth citation:'
require_text "$proposal" 'Safe suggestion policy:'

require_text "$ambiguity" 'Status: `open`'
require_text "$ambiguity" 'Blocking milestone: `M0016`'
require_text "$task" 'Status: `complete`'

require_absent_path docs/adr/ADR-0026-name-resolution-policy.md
require_absent_text docs/SPEC.md '^## ADR-0026: Name Resolution Policy$'
require_absent_path crates/newlang/src/name_resolution.rs
require_absent_path crates/newlang/src/resolution.rs
require_absent_text crates/newlang/src/lib.rs 'pub mod name_resolution|pub mod resolution'

echo "m0016-concrete-draft: concrete name resolution draft validation passed"
