#!/usr/bin/env sh
set -eu

fail() {
  echo "m0013-concrete-draft: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope path exists during concrete draft task: $1"
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

proposal=docs/adr/proposals/ADR-0024-expression-statement-pattern-syntax.md
ambiguity=docs/ambiguities/M0008-expression-statement-pattern-syntax.md
decision=docs/adr/proposals/reviews/ADR-0024-chief-architect-decision.md
task=docs/tasks/M0013-004-expression-statement-pattern-concrete-draft.md

require_file "$proposal"
require_file "$ambiguity"
require_file "$decision"
require_file "$task"

require_text "$proposal" '^Status: Draft proposal - not accepted source of truth$'
require_text "$proposal" '^## Concrete Draft Grammar$'
require_text "$proposal" '^### Body Grammar Overview$'
require_text "$proposal" '^### Block Syntax$'
require_text "$proposal" '^### Statement Syntax$'
require_text "$proposal" '^### Expression Grammar$'
require_text "$proposal" '^### Operator Precedence And Associativity$'
require_text "$proposal" '^### Pattern Grammar$'
require_text "$proposal" '^### Unsafe And Coroutine Syntax$'
require_text "$proposal" '^### Recovery Boundaries$'
require_text "$proposal" '^### Parser Diagnostics$'
require_text "$proposal" '^### Review Attack Cases$'
require_text "$proposal" '^### Concrete Deferrals$'
require_text "$proposal" 'block = `\{` statement\* expression\? `\}`'
require_text "$proposal" 'statement = variable-declaration \| assignment-statement \| return-statement \| expression-statement'
require_text "$proposal" 'expression = assignment-expression'
require_text "$proposal" 'pattern = wildcard-pattern \| literal-pattern \| binding-pattern \| qualified-case-pattern \| grouped-pattern'
require_text "$proposal" '\| Precedence \| Operators \| Associativity \|'
require_text "$proposal" 'unsafe block syntax is deferred'
require_text "$proposal" 'coroutine syntax is deferred'
require_text "$proposal" 'missing_expression'
require_text "$proposal" 'malformed_pattern'
require_text "$proposal" 'malformed_block'
require_text "$proposal" 'primary span'
require_text "$proposal" 'recovery action'
require_text "$proposal" 'All accepted body syntax diagnostics must cite ADR-0015 and ADR-0024'

require_text "$ambiguity" 'Status: `resolved`'
require_text "$ambiguity" 'docs/adr/ADR-0024-expression-statement-pattern-syntax.md'
require_text "$decision" '^Decision: approved$'
require_text "$decision" 'Accepted source of truth: `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`'
require_text "$task" 'Status: `complete`'

require_file docs/adr/ADR-0024-expression-statement-pattern-syntax.md
require_text docs/adr/ADR-0024-expression-statement-pattern-syntax.md '^Status: Accepted$'
require_absent_path tests/fixtures/parser/expressions
require_absent_path tests/fixtures/parser/statements
require_absent_path tests/fixtures/parser/patterns
require_text docs/SPEC.md '^## ADR-0024: Expression Statement And Pattern Syntax$'
require_absent_text crates/newlang/src/parser.rs 'parse_expression|parse_statement|parse_pattern|parse_block|parse_when|parse_match|parse_coroutine|parse_unsafe'
require_absent_text crates/newlang/src/ast.rs 'Expression|Statement|Pattern|Block|When|Match|UnsafeBlock|Coroutine'

echo "m0013-concrete-draft: expression statement and pattern concrete draft validation passed"
