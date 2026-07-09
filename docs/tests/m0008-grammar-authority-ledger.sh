#!/usr/bin/env sh
set -eu

fail() {
  echo "m0008-ledger: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope path exists during M0008 planning: $1"
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

ledger=docs/syntax/grammar-authority-ledger.md
task=docs/tasks/M0008-001-grammar-authority-ledger.md
milestone=docs/milestones/M0008-grammar-authority-and-syntax-ambiguity-ledger.md

require_file "$ledger"
require_file "$task"
require_file "$milestone"
require_file docs/ambiguities/M0008-declaration-syntax.md
require_file docs/ambiguities/M0008-type-generic-syntax.md
require_file docs/ambiguities/M0008-expression-statement-pattern-syntax.md

require_text "$ledger" '^# Grammar Authority Ledger$'
require_text "$ledger" 'Status: M0008 authority ledger'
require_text "$ledger" 'Source of truth: `docs/SPEC.md` and accepted ADRs under `docs/adr/`'
require_text "$ledger" '\| Construct \| Classification \| Authority \| Owner \| Blocking milestone \| Notes \|'
require_text "$ledger" '\| Package declaration \| ambiguous \| ADR-0017'
require_text "$ledger" '\| Import declaration \| ambiguous \| ADR-0017'
require_text "$ledger" '\| Function declaration \| ambiguous \| ADR-0010'
require_text "$ledger" '\| Type declaration \| ambiguous \| ADR-0010'
require_text "$ledger" '\| Generic parameter syntax \| ambiguous \| ADR-0016'
require_text "$ledger" '\| Nullable type syntax \| ambiguous \| ADR-0006'
require_text "$ledger" '\| Expression grammar \| ambiguous \| none'
require_text "$ledger" '\| Pattern grammar \| ambiguous \| ADR-0012'
require_text "$ledger" '\| Token spellings \| specified \| ADR-0021'
require_text "$ledger" '^## Parser Unblock List$'
require_text "$ledger" 'Only token-consuming parser infrastructure may proceed before syntax ADRs'
require_text "$ledger" '^## Parser Block List$'
require_text "$ledger" 'M0011'
require_text "$ledger" 'M0012'
require_text "$ledger" 'M0013'

for report in \
  docs/ambiguities/M0008-declaration-syntax.md \
  docs/ambiguities/M0008-type-generic-syntax.md \
  docs/ambiguities/M0008-expression-statement-pattern-syntax.md
do
  require_text "$report" 'Status: `open`'
  require_text "$report" 'Required Owner: `Language Designer`'
  require_text "$report" 'Why guessing Is Unsafe'
  require_text "$report" 'Blocking milestone'
done

require_text "$milestone" '\[x\] Syntax constructs are classified'
require_text "$milestone" '\[x\] Blocking ambiguities are recorded'
require_text "$milestone" '\[x\] Parser scope is defined'

require_absent_path crates/newlang/src/parser.rs
require_absent_path tests/fixtures/parser

echo "m0008-ledger: grammar authority ledger validation passed"
