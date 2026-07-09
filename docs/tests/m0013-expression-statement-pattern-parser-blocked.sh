#!/usr/bin/env sh
set -eu

fail() {
  echo "m0013-blocked: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope path exists while M0013 is blocked: $1"
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

task=docs/tasks/M0013-001-expression-statement-pattern-syntax-blocker.md
ambiguity=docs/ambiguities/M0008-expression-statement-pattern-syntax.md
ledger=docs/syntax/grammar-authority-ledger.md
milestone=docs/milestones/M0013-expression-statement-and-pattern-parser.md

require_file "$task"
require_file "$ambiguity"
require_file "$ledger"
require_file "$milestone"
require_file crates/newlang/src/parser.rs
require_file crates/newlang/src/ast.rs

require_text "$task" '^# Task: M0013-001 Record expression statement and pattern syntax blocker$'
require_text "$task" 'Status: `complete`'
require_text "$task" 'Milestone: `M0013`'
require_text "$task" 'Language Designer to draft a non-authoritative syntax proposal'

require_text "$ambiguity" 'Status: `resolved`'
require_text "$ambiguity" 'Blocking milestone: `M0013`'
require_text "$ambiguity" 'Parser implementation may accept only the concrete expression, statement, and pattern syntax defined by ADR-0024'
require_text "$ambiguity" 'Expression precedence'
require_text "$ambiguity" 'Pattern syntax affects exhaustiveness'
require_text "$ambiguity" 'Coroutine and unsafe syntax affect safety boundaries'

require_text "$ledger" '\| Expression grammar \| specified \| ADR-0024'
require_text "$ledger" '\| Statement grammar \| specified \| ADR-0024'
require_text "$ledger" '\| Pattern grammar \| specified \| ADR-0024'
require_text "$ledger" '\| Coroutine syntax \| deferred \| ADR-0024'
require_text "$ledger" '\| Unsafe block syntax \| deferred \| ADR-0024'
require_text "$ledger" 'M0013 expression, statement, and pattern parser may proceed only for ADR-0024 constructs'

require_text "$milestone" 'M0013'
require_text "$milestone" '\[x\] Ambiguous syntax remains blocked'
require_text "$milestone" '\[ \] Expression fixtures pass'
require_text "$milestone" '\[ \] Statement fixtures pass'
require_text "$milestone" '\[ \] Pattern fixtures pass'

require_absent_text crates/newlang/src/parser.rs 'parse_expression|parse_statement|parse_pattern|parse_block|parse_when|parse_match|parse_coroutine|parse_unsafe'
require_absent_text crates/newlang/src/ast.rs 'Expression|Statement|Pattern|Block|When|Match|UnsafeBlock|Coroutine'
require_absent_path tests/fixtures/parser/expressions
require_absent_path tests/fixtures/parser/statements
require_absent_path tests/fixtures/parser/patterns

echo "m0013-authority: expression statement and pattern syntax authority resolved while implementation remains deferred"
