#!/usr/bin/env sh
set -eu

fail() {
  echo "m0011-blocked: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope path exists while M0011 is blocked: $1"
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

task=docs/tasks/M0011-001-declaration-syntax-blocker.md
ambiguity=docs/ambiguities/M0008-declaration-syntax.md
ledger=docs/syntax/grammar-authority-ledger.md
milestone=docs/milestones/M0011-declaration-parser.md

require_file "$task"
require_file "$ambiguity"
require_file "$ledger"
require_file "$milestone"

require_text "$task" '^# Task: M0011-001 Record Declaration Parser Syntax Blocker$'
require_text "$task" 'Status: `blocked`'
require_text "$task" 'Milestone: `M0011`'
require_text "$task" 'Language Designer drafts declaration syntax ADR'
require_text "$ambiguity" 'Status: `open`'
require_text "$ambiguity" 'Blocking milestone: `M0011`'
require_text "$ambiguity" 'No parser implementation may accept concrete declaration syntax'
require_text "$ledger" '\| Package declaration \| ambiguous \| ADR-0017'
require_text "$ledger" '\| Function declaration \| ambiguous \| ADR-0010'
require_text "$ledger" 'M0011 declaration parser is blocked on declaration syntax authority'

require_absent_path crates/newlang/src/parser.rs
require_absent_path tests/fixtures/parser

echo "m0011-blocked: declaration parser correctly blocked by declaration syntax ambiguity"
