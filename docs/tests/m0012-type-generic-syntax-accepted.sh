#!/usr/bin/env sh
set -eu

fail() {
  echo "m0012-accepted: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope path exists during ADR acceptance: $1"
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

adr=docs/adr/ADR-0023-type-and-generic-syntax.md
spec=docs/SPEC.md
ambiguity=docs/ambiguities/M0008-type-generic-syntax.md
decision=docs/adr/proposals/reviews/ADR-0023-chief-architect-decision.md
ledger=docs/syntax/grammar-authority-ledger.md
task=docs/tasks/M0012-005-accept-type-generic-syntax-adr.md

require_file "$adr"
require_file "$spec"
require_file "$ambiguity"
require_file "$decision"
require_file "$ledger"
require_file "$task"

require_text "$adr" '^# ADR-0023: Type And Generic Syntax$'
require_text "$adr" '^Status: Accepted$'
require_text "$adr" '^## Concrete Grammar$'
require_text "$adr" '^### Type Grammar Overview$'
require_text "$adr" '^### Nullable Type Syntax$'
require_text "$adr" '^### Generic Parameter Syntax$'
require_text "$adr" '^### Generic Argument Syntax$'
require_text "$adr" '^### Capability-Bound Syntax$'
require_text "$adr" '^### Function Type Syntax$'
require_text "$adr" '^### Type Syntax Diagnostics$'
require_text "$adr" 'type = nullable-type'
require_text "$adr" 'nullable-type = primary-type `\?`\?'
require_text "$adr" 'capability-bound-list = capability-bound \(`&` capability-bound\)\*'
require_text "$adr" 'function-type = `\(` function-type-parameters\? `\)` `->` type'
require_text "$adr" 'missing_type_name'
require_text "$adr" 'malformed_capability_bound'
require_text "$adr" 'All type syntax diagnostics must cite ADR-0015 and ADR-0023'

require_text "$spec" '^## ADR-0023: Type And Generic Syntax$'
require_text "$spec" 'named type references'
require_text "$spec" 'nullable type syntax'
require_text "$spec" 'generic parameter syntax'
require_text "$spec" 'capability-bound syntax'

require_text "$ambiguity" 'Status: `resolved`'
require_text "$ambiguity" 'docs/adr/ADR-0023-type-and-generic-syntax.md'
require_text "$ambiguity" '2026-07-10'
require_absent_text "$ambiguity" 'unresolved'

require_text "$decision" '^Decision: approved$'
require_text "$decision" 'Accepted source of truth: `docs/adr/ADR-0023-type-and-generic-syntax.md`'
require_text "$decision" 'M0012 type and generic parser fixture and implementation tasks may proceed'

require_text "$ledger" '\| Type declaration \| specified \| ADR-0023'
require_text "$ledger" '\| Generic parameter syntax \| specified \| ADR-0023'
require_text "$ledger" '\| Generic argument syntax \| specified \| ADR-0023'
require_text "$ledger" '\| Capability bound syntax \| specified \| ADR-0023'
require_text "$ledger" '\| Nullable type syntax \| specified \| ADR-0023'
require_text "$ledger" '\| Function type syntax \| specified \| ADR-0023'
require_text "$ledger" '\| Expression grammar \| ambiguous \| none'
require_text "$ledger" '\| Statement grammar \| ambiguous \| none'
require_text "$ledger" '\| Pattern grammar \| ambiguous \| ADR-0012'
require_text "$ledger" 'M0012 type and generic parser may proceed only for ADR-0023 constructs'

require_text "$task" 'Status: `complete`'
require_text "$task" 'Tests fail before implementation for the expected reason'
require_text "$task" 'CI passes as final gate'

echo "m0012-accepted: type and generic syntax authority validation passed"
