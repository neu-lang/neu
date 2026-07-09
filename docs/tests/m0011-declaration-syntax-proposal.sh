#!/usr/bin/env sh
set -eu

fail() {
  echo "m0011-proposal-history: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope parser artifact exists during proposal history validation: $1"
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

proposal=docs/adr/proposals/ADR-0022-declaration-syntax.md
ambiguity=docs/ambiguities/M0008-declaration-syntax.md
task=docs/tasks/M0011-002-declaration-syntax-proposal.md

require_file "$proposal"
require_file "$ambiguity"
require_file "$task"

require_text "$proposal" '^# ADR-0022: Declaration Syntax$'
require_text "$proposal" '^Status: Draft proposal - not accepted source of truth$'
require_text "$proposal" '^## Non-Authority Notice$'
require_text "$proposal" '^## Question$'
require_text "$proposal" '^## Competing Designs$'
require_text "$proposal" '^## Trade-offs$'
require_text "$proposal" '^## Recommended Draft Choice$'
require_text "$proposal" '^## Required Accepted Content$'
require_text "$proposal" '^## Downstream Consequences$'
require_text "$proposal" '^## Dependencies$'
require_text "$proposal" 'small Kotlin-like custom declaration grammar'
require_text "$proposal" 'No parser implementation may depend on this proposal until accepted'
require_text "$proposal" 'package declarations'
require_text "$proposal" 'import declarations'
require_text "$proposal" 'function declarations'
require_text "$proposal" 'struct declarations'
require_text "$proposal" 'interface declarations'
require_text "$proposal" 'declaration diagnostics'

require_text "$ambiguity" 'Status: `resolved`'
require_text "$ambiguity" 'Blocking milestone: `M0011`'
require_text "$ambiguity" 'docs/adr/ADR-0022-declaration-syntax.md'
require_text "$task" 'Milestone: `M0011`'


echo "m0011-proposal-history: declaration syntax ADR proposal history validation passed"
