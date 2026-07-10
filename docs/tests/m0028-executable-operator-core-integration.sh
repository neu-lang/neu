#!/usr/bin/env sh
set -eu

fail() {
  echo "m0028-executable-operator-core-integration: $*" >&2
  exit 1
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

task=docs/tasks/M0028-003-executable-operator-core-integration.md

[ -f "$task" ] || fail "missing task file"
require_text "$task" 'Milestone: `M0028`'
require_text crates/compiler/src/type_check.rs 'type_m0028_executable_core'
require_text crates/compiler/tests/type_check.rs 'm0028_core_types_executable_operators_before_initializers_and_assignments'
require_text crates/compiler/tests/type_check.rs 'm0028_core_rejects_non_int_operator_operands_without_generic_deferral'
require_text crates/compiler/tests/type_check.rs 'm0028_core_keeps_non_executable_operators_deferred'

cargo test -p compiler --test type_check

echo "m0028 executable operator core integration validation passed"
