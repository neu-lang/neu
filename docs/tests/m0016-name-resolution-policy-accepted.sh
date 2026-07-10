#!/usr/bin/env sh
set -eu

fail() {
  echo "m0016-accepted: $*" >&2
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

adr=docs/adr/ADR-0026-name-resolution-policy.md
spec=docs/SPEC.md
ambiguity=docs/ambiguities/M0016-name-resolution-policy.md
decision=docs/adr/proposals/reviews/ADR-0026-chief-architect-decision.md
milestone=docs/milestones/M0016-name-resolution-pass.md
task=docs/tasks/M0016-005-accept-name-resolution-policy-adr.md

require_file "$adr"
require_file "$spec"
require_file "$ambiguity"
require_file "$decision"
require_file "$milestone"
require_file "$task"

require_text "$adr" '^# ADR-0026: Name Resolution Policy$'
require_text "$adr" '^Status: Accepted$'
require_text "$adr" '^## Decision$'
require_text "$adr" 'local lexical scope plus same-module package top-level declarations'
require_text "$adr" '^## Resolvable AST Node Kinds$'
require_text "$adr" '^## Declaration And Binding Positions$'
require_text "$adr" '^## Scope And Declaration Order$'
require_text "$adr" '^## Shadowing And Duplicate Rules$'
require_text "$adr" '^## Lookup Rules$'
require_text "$adr" '^## Visibility Rule$'
require_text "$adr" '^## Required Diagnostics$'
require_text "$adr" '^## Explicit Deferrals$'
require_text "$adr" 'simple identifier expression'
require_text "$adr" 'qualified name expression'
require_text "$adr" 'type name node'
require_text "$adr" 'function declaration name'
require_text "$adr" 'type declaration name'
require_text "$adr" 'local `val` statement'
require_text "$adr" 'local `var` statement'
require_text "$adr" 'pattern bindings are excluded'
require_text "$adr" 'function parameters are excluded'
require_text "$adr" 'local bindings are not visible before their declaration statement'
require_text "$adr" 'same-scope duplicate local binding'
require_text "$adr" 'same-module same-package duplicate top-level declaration'
require_text "$adr" 'ambiguity instead of choosing by insertion order'
require_text "$adr" 'imports remain syntax-only'
require_text "$adr" 'cross-module lookup remains unsupported'
require_text "$adr" 'member lookup remains unsupported'
require_text "$adr" 'overload resolution remains unsupported'
require_text "$adr" 'extension method lookup remains unsupported'
require_text "$adr" 'type-directed lookup remains unsupported'
require_text "$adr" 'Diagnostic: `unresolved_name`'
require_text "$adr" 'Diagnostic: `duplicate_name`'
require_text "$adr" 'Diagnostic: `ambiguous_name`'
require_text "$adr" 'Diagnostic: `unsupported_import_resolution`'
require_text "$adr" 'Primary span:'
require_text "$adr" 'Recovery action:'
require_text "$adr" 'Source-of-truth citation:'
require_text "$adr" 'Safe suggestion policy:'
require_absent_text "$adr" '^Status: Draft proposal'
require_absent_text "$adr" '^## Non-Authority Notice$'
require_absent_text "$adr" 'not accepted source of truth'
require_absent_text "$adr" 'This section is a draft direction'

require_text "$spec" '^## ADR-0026: Name Resolution Policy$'
require_text "$spec" 'M0016 resolves a bootstrap subset using local lexical scope plus same-module package top-level declarations'
require_text "$spec" 'Imports remain syntax-only and do not add lookup candidates'
require_text "$spec" 'Cross-module lookup, member lookup, overload resolution, extension lookup, and type-directed lookup remain unsupported'
require_text "$spec" 'Resolution diagnostics include `unresolved_name`, `duplicate_name`, `ambiguous_name`, `unsupported_import_resolution`, `unsupported_cross_module_lookup`, and `unsupported_member_resolution`'

require_text "$ambiguity" 'Status: `resolved`'
require_text "$ambiguity" 'Resolution Source: `docs/adr/ADR-0026-name-resolution-policy.md`'
require_text "$ambiguity" 'Resolved Date: `2026-07-10`'
require_text "$ambiguity" '\[x\] Language Lawyer determines whether existing text resolves it'
require_text "$ambiguity" '\[x\] Adversarial Engineer reviews soundness risk'
require_text "$ambiguity" '\[x\] Diagnostics Engineer reviews unresolved-name and duplicate-name diagnostics'
require_text "$ambiguity" '\[x\] Simplicity Guardian reviews the bootstrap subset for overreach'
require_text "$ambiguity" '\[x\] Chief Architect approves final source-of-truth update'
require_text "$ambiguity" 'Implementation may define name resolution only as specified by accepted ADR-0026'

require_text "$decision" '^Decision: approved$'
require_text "$decision" 'ADR-0026 is accepted'
require_text "$decision" 'M0016 ambiguity is resolved'

require_text "$milestone" '\[x\] Name resolution source of truth is accepted'
require_text "$task" 'Status: `complete`'

require_file crates/newlang/src/name_resolution.rs
require_absent_path crates/newlang/src/resolution.rs
require_text crates/newlang/src/lib.rs 'pub mod name_resolution;'
require_absent_text crates/newlang/src/lib.rs 'pub mod resolution'
require_absent_text crates/newlang/src/name_resolution.rs 'LookupScope|ScopeStack|ImportResolver|VisibilityEnforcement|resolve_names|resolve_module|resolve_file'
require_absent_text crates/newlang/src/parser.rs 'NameResolution|ResolvedName|UnresolvedName|ImportResolver|LookupScope'

echo "m0016-accepted: name resolution policy accepted ADR validation passed"
