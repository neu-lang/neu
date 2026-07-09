#!/usr/bin/env sh
set -eu

fail() {
  echo "m0012-authority: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope path exists while M0012 is blocked: $1"
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

task=docs/tasks/M0012-001-type-generic-syntax-blocker.md
ambiguity=docs/ambiguities/M0008-type-generic-syntax.md
ledger=docs/syntax/grammar-authority-ledger.md
milestone=docs/milestones/M0012-type-and-generic-syntax-parser.md

require_file "$task"
require_file "$ambiguity"
require_file "$ledger"
require_file "$milestone"
require_file crates/newlang/src/parser.rs
require_file crates/newlang/tests/parser.rs

require_text "$task" '^# Task: M0012-001 Record type and generic syntax parser blocker$'
require_text "$task" 'Status: `blocked`'
require_text "$task" 'Milestone: `M0012`'
require_text "$task" 'Language Designer drafts type and generic syntax ADR'

require_text "$ambiguity" 'Status: `resolved`'
require_text "$ambiguity" 'Blocking milestone: `M0012`'
require_text "$ambiguity" 'Parser implementation may accept only the concrete type and generic syntax defined by ADR-0023'

require_text "$ledger" '\| Type declaration \| specified \| ADR-0023'
require_text "$ledger" '\| Generic parameter syntax \| specified \| ADR-0023'
require_text "$ledger" '\| Generic argument syntax \| specified \| ADR-0023'
require_text "$ledger" '\| Capability bound syntax \| specified \| ADR-0023'
require_text "$ledger" '\| Nullable type syntax \| specified \| ADR-0023'
require_text "$ledger" '\| Function type syntax \| specified \| ADR-0023'
require_text "$ledger" 'M0012 type and generic parser may proceed only for ADR-0023 constructs'

require_text "$milestone" 'M0012'
require_text "$milestone" '\[x\] Ambiguities are recorded'
require_text "$milestone" '\[x\] Approved type syntax parses'

echo "m0012-authority: type and generic syntax authority and implementation validation passed"
