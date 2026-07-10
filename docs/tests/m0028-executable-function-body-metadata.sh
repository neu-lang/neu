#!/usr/bin/env sh
set -eu

fail() {
  echo "m0028-executable-function-body-metadata: $*" >&2
  exit 1
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

task=docs/tasks/M0028-006-executable-function-body-metadata.md

[ -f "$task" ] || fail "missing task file"
require_text "$task" 'Milestone: `M0028`'
require_text crates/compiler/src/parser.rs 'ParsedFunctionDeclaration'
require_text crates/compiler/src/parser.rs 'ParsedReturnStatement'
require_text crates/compiler/src/parser.rs 'ParsedCallExpression'
require_text crates/compiler/tests/parser.rs 'm0028_records_executable_function_return_and_call_metadata'
require_text crates/compiler/tests/parser.rs 'm0028_executable_metadata_excludes_malformed_function_and_call_records'

cargo test -p compiler --test parser m0028_

printf '%s\n' 'm0028 executable function body metadata validation passed'
