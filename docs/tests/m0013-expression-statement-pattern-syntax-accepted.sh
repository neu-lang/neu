#!/usr/bin/env sh
set -eu

fail() {
  echo "m0013-accepted: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
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

adr=docs/adr/ADR-0024-expression-statement-pattern-syntax.md
spec=docs/SPEC.md
ambiguity=docs/ambiguities/M0008-expression-statement-pattern-syntax.md
decision=docs/adr/proposals/reviews/ADR-0024-chief-architect-decision.md
ledger=docs/syntax/grammar-authority-ledger.md
task=docs/tasks/M0013-005-accept-expression-statement-pattern-syntax-adr.md

require_file "$adr"
require_file "$spec"
require_file "$ambiguity"
require_file "$decision"
require_file "$ledger"
require_file "$task"

require_text "$adr" '^# ADR-0024: Expression Statement And Pattern Syntax$'
require_text "$adr" '^Status: Accepted$'
require_text "$adr" '^## Concrete Grammar$'
require_text "$adr" '^### Body Grammar Overview$'
require_text "$adr" '^### Block Syntax$'
require_text "$adr" '^### Statement Syntax$'
require_text "$adr" '^### Expression Grammar$'
require_text "$adr" '^### Operator Precedence And Associativity$'
require_text "$adr" '^### Pattern Grammar$'
require_text "$adr" '^### Unsafe And Coroutine Syntax$'
require_text "$adr" '^### Recovery Boundaries$'
require_text "$adr" '^### Parser Diagnostics$'
require_text "$adr" '^### Review Attack Cases$'
require_text "$adr" '^### Concrete Deferrals$'
require_text "$adr" 'block = `\{` statement\* expression\? `\}`'
require_text "$adr" 'statement = variable-declaration \| assignment-statement \| return-statement \| expression-statement'
require_text "$adr" 'expression = assignment-expression'
require_text "$adr" 'pattern = wildcard-pattern \| literal-pattern \| binding-pattern \| qualified-case-pattern \| grouped-pattern'
require_text "$adr" '\| Precedence \| Operators \| Associativity \|'
require_text "$adr" 'unsafe block syntax is deferred'
require_text "$adr" 'coroutine syntax is deferred'
require_text "$adr" 'missing_expression'
require_text "$adr" 'malformed_pattern'
require_text "$adr" 'All accepted body syntax diagnostics must cite ADR-0015 and ADR-0024'

require_text "$spec" '^## ADR-0024: Expression Statement And Pattern Syntax$'
require_text "$spec" 'expression grammar'
require_text "$spec" 'statement grammar'
require_text "$spec" 'pattern grammar'
require_text "$spec" 'operator precedence'

require_text "$ambiguity" 'Status: `resolved`'
require_text "$ambiguity" 'docs/adr/ADR-0024-expression-statement-pattern-syntax.md'
require_text "$ambiguity" '2026-07-10'
require_absent_text "$ambiguity" 'unresolved'

require_text "$decision" '^Decision: approved$'
require_text "$decision" 'Accepted source of truth: `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`'
require_text "$decision" 'M0013 body parser fixture and implementation tasks may proceed'

require_text "$ledger" '\| Expression grammar \| specified \| ADR-0024'
require_text "$ledger" '\| Statement grammar \| specified \| ADR-0024'
require_text "$ledger" '\| Pattern grammar \| specified \| ADR-0024'
require_text "$ledger" '\| Coroutine syntax \| deferred \| ADR-0024'
require_text "$ledger" '\| Unsafe block syntax \| deferred \| ADR-0024'
require_text "$ledger" 'M0013 expression, statement, and pattern parser may proceed only for ADR-0024 constructs'

require_text "$task" 'Status: `complete`'
require_text "$task" 'Tests fail before implementation for the expected reason'
require_text "$task" 'CI passes as final gate'

require_absent_text crates/newlang/src/parser.rs 'parse_expression|parse_statement|parse_pattern|parse_block|parse_when|parse_match|parse_coroutine|parse_unsafe'
require_absent_text crates/newlang/src/ast.rs 'Expression|Statement|Pattern|Block|When|Match|UnsafeBlock|Coroutine'

echo "m0013-accepted: expression statement and pattern syntax accepted and parser implementation still deferred"
