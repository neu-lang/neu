#!/usr/bin/env sh
set -eu

fail() {
  echo "m0016-authority: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope path exists while M0016 is blocked: $1"
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

task=docs/tasks/M0016-001-name-resolution-policy-blocker.md
ambiguity=docs/ambiguities/M0016-name-resolution-policy.md
milestone=docs/milestones/M0016-name-resolution-pass.md
spec=docs/SPEC.md

require_file "$task"
require_file "$ambiguity"
require_file "$milestone"
require_file "$spec"

require_text "$task" '^# Task: M0016-001 Record name resolution policy blocker$'
require_text "$task" 'Status: `complete`'
require_text "$task" 'Milestone: `M0016`'

require_text "$ambiguity" '^# Ambiguity Report: M0016 Name Resolution Policy$'
require_text "$ambiguity" 'Report ID: `M0016-NAME-RESOLUTION-POLICY`'
require_text "$ambiguity" 'Related Task: `M0016-001`'
require_text "$ambiguity" 'Status: `resolved`'
require_text "$ambiguity" 'Required Owner: `main-task semantic design`'
require_text "$ambiguity" 'Resolution Source: `docs/adr/ADR-0026-name-resolution-policy.md`'
require_text "$ambiguity" 'lookup order'
require_text "$ambiguity" 'import semantics'
require_text "$ambiguity" 'scope boundaries'
require_text "$ambiguity" 'duplicate-name behavior'
require_text "$ambiguity" 'unresolved-name diagnostics'
require_text "$ambiguity" 'Implementation may define name resolution only as specified by accepted ADR-0026'

require_text "$milestone" 'M0016'
require_text "$milestone" '\[x\] Name resolution source of truth is accepted'
require_text "$milestone" '\[x\] Approved names resolve'
require_text "$milestone" '\[x\] Unresolved names diagnose'
require_text "$milestone" '\[x\] Ambiguous resolution cases are not guessed'

require_text "$spec" '^## ADR-0025: Module Package And Visibility Model$'
require_text "$spec" 'later dependency and name resolution rules'
require_text "$spec" '^## ADR-0026: Name Resolution Policy$'

require_file crates/compiler/src/name_resolution.rs
require_absent_path crates/compiler/src/resolution.rs
require_file crates/compiler/tests/name_resolution.rs
require_text crates/compiler/src/lib.rs 'pub mod name_resolution;'
require_absent_text crates/compiler/src/lib.rs 'pub mod resolution'
require_absent_text crates/compiler/src/name_resolution.rs 'LookupScope|ScopeStack|ImportResolver|VisibilityEnforcement|resolve_names|resolve_module|resolve_file'
require_absent_text crates/compiler/src/symbol.rs 'ImportResolver|VisibilityEnforcement|LookupDiagnostic|ScopeStack|ResolutionPolicy'
require_absent_text crates/compiler/src/parser.rs 'NameResolution|UnresolvedName|ResolvedName|ImportResolver'

echo "m0016-authority: name resolution policy authority validation passed"
