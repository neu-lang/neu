#!/usr/bin/env sh
set -eu

fail() {
  echo "m0028-entry-point-candidate-validation: $*" >&2
  exit 1
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

task=docs/tasks/M0028-007-entry-point-candidate-validation.md

[ -f "$task" ] || fail "missing task file"
require_text "$task" 'Milestone: `M0028`'
require_text docs/adr/ADR-0049-bootstrap-entry-point-diagnostic-provenance.md '^Status: Accepted$'
require_text docs/SPEC.md 'ADR-0049: Bootstrap Entry-Point Diagnostic Provenance'
require_text crates/compiler/src/type_check.rs 'check_m0028_entry_point'
require_text crates/compiler/tests/type_check.rs 'm0028_entry_point_selects_one_valid_main_in_the_explicit_package'
require_text crates/compiler/tests/type_check.rs 'm0028_entry_point_rejects_every_non_entry_main_shape'

cargo test -p compiler --test type_check m0028_entry_point

printf '%s\n' 'm0028 entry-point candidate validation passed'
