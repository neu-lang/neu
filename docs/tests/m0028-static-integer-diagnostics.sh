#!/usr/bin/env sh
set -eu

fail() {
  echo "m0028-static-integer-diagnostics: $*" >&2
  exit 1
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

task=docs/tasks/M0028-005-static-integer-diagnostics.md
adr=docs/adr/ADR-0048-bootstrap-integer-constant-expressions.md

[ -f "$task" ] || fail "missing task file"
[ -f "$adr" ] || fail "missing accepted ADR-0048"
require_text "$task" 'Milestone: `M0028`'
require_text "$adr" '^Status: Accepted$'
require_text docs/SPEC.md 'ADR-0048: Bootstrap Integer Constant Expressions'
require_text docs/milestones/M0028-executable-expression-frontend-completion.md 'Bootstrap constant-tree integer diagnostics exist'
require_text docs/ambiguities/M0028-static-integer-constant-expressions.md 'Status: `resolved`'
require_text crates/compiler/src/type_check.rs 'type_m0028_static_integer_diagnostics'
require_text crates/compiler/tests/type_check.rs 'm0028_static_integer_diagnostics_cover_adr0043_failures'
require_text crates/compiler/tests/type_check.rs 'm0028_static_integer_diagnostics_accept_every_bootstrap_integer_operator'

cargo test -p compiler --test type_check m0028_static_integer_diagnostics

printf '%s\n' 'm0028 static integer diagnostics validation passed'
