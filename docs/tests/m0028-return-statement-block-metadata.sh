#!/usr/bin/env sh
set -eu

fail() {
  echo "m0028-return-statement-block-metadata: $*" >&2
  exit 1
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

task=docs/tasks/M0028-008-return-statement-block-metadata.md

[ -f "$task" ] || fail "missing task file"
require_text "$task" 'Milestone: `M0028`'
require_text crates/compiler/src/parser.rs 'block_return_indices'
require_text crates/compiler/src/parser.rs 'pub block: AstNodeId'
require_text crates/compiler/tests/parser.rs 'm0028_records_return_statement_enclosing_blocks_in_source_order'

cargo test -p compiler --test parser m0028_records_return_statement_enclosing_blocks_in_source_order

printf '%s\n' 'm0028 return statement block metadata validation passed'
