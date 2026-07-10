#!/usr/bin/env sh
set -eu

fail() {
  echo "m0018-accepted: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq -- "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

require_absent_text() {
  file="$1"
  pattern="$2"
  if grep -Eq -- "$pattern" "$file"; then
    fail "unexpected pattern in $file: $pattern"
  fi
}

adr=docs/adr/ADR-0027-type-checking-core.md
spec=docs/SPEC.md
ambiguity=docs/ambiguities/M0018-type-checking-core.md
decision=docs/adr/proposals/reviews/ADR-0027-chief-architect-decision.md
task=docs/tasks/M0018-005-accept-type-checking-core-adr.md
source=crates/newlang/src/type_check.rs

require_file "$adr"
require_file "$spec"
require_file "$ambiguity"
require_file "$decision"
require_file "$task"
require_file "$source"

require_text "$adr" '^# ADR-0027: Type Checking Core$'
require_text "$adr" '^Status: Accepted$'
require_text "$adr" '^## Decision$'
require_text "$adr" 'small bootstrap type checker with primitive type-checking identities'
require_text "$adr" '^## Concrete Type Checking Model$'
require_text "$adr" '^## Typed Output Shape$'
require_text "$adr" '^## Primitive Type Identity$'
require_text "$adr" '^## Included Expression Forms$'
require_text "$adr" '^## Assignment Compatibility$'
require_text "$adr" '^## Direct Call Deferral$'
require_text "$adr" '^## Function Type Application Deferral$'
require_text "$adr" '^## Type Checking Diagnostics$'
require_text "$adr" '^## Unsupported And Ambiguous Rules$'
require_text "$adr" '^## Explicit Deferrals$'
require_text "$adr" 'expression type table'
require_text "$adr" 'declaration signature table'
require_text "$adr" 'assignment check table'
require_text "$adr" 'diagnostics list'
require_text "$adr" 'no typed AST rewrite'
require_text "$adr" 'PrimitiveType'
require_text "$adr" 'type-checking identity only'
require_text "$adr" 'no ABI or layout meaning'
require_text "$adr" 'Non-null base values are assignment-compatible with their nullable wrapper'
require_text "$adr" '`Null` is assignment-compatible only with nullable target types'
require_text "$adr" 'Direct function declaration calls are deferred for M0018'
require_text "$adr" 'Structural function type application is deferred for M0018'
require_text "$adr" 'Diagnostic: `type_mismatch`'
require_text "$adr" 'Diagnostic: `unresolved_type_rule`'
require_text "$adr" 'Diagnostic: `unsupported_type_rule`'
require_text "$adr" 'Diagnostic: `ambiguous_type_rule`'
require_text "$adr" 'Primary span:'
require_text "$adr" 'Recovery action:'
require_text "$adr" 'Source-of-truth citation:'
require_text "$adr" 'Safe suggestion policy:'
require_text "$adr" 'stable rule identifier'
require_text "$adr" 'overload resolution'
require_text "$adr" 'implicit numeric conversion'
require_text "$adr" 'generic constraint solving'
require_text "$adr" 'ownership and move analysis'
require_text "$adr" 'borrow checking'
require_text "$adr" 'HIR lowering'
require_text "$adr" 'MIR lowering'
require_absent_text "$adr" '^Status: Draft proposal'
require_absent_text "$adr" '^## Non-Authority Notice$'
require_absent_text "$adr" 'not accepted source of truth'
require_absent_text "$adr" 'This section is a draft direction'

require_text "$spec" '^## ADR-0027: Type Checking Core$'
require_text "$spec" 'M0018 defines a small bootstrap type checker with primitive type-checking identities'
require_text "$spec" 'Typed output is side-table metadata'
require_text "$spec" 'Direct function declaration calls and structural function type application are deferred for M0018'
require_text "$spec" 'Type checking diagnostics include `type_mismatch`, `unresolved_type_rule`, `unsupported_type_rule`, and `ambiguous_type_rule`'

require_text "$ambiguity" 'Status: `resolved`'
require_text "$ambiguity" 'Resolution Source: `docs/adr/ADR-0027-type-checking-core.md`'
require_text "$ambiguity" 'Resolved Date: `2026-07-10`'
require_text "$ambiguity" 'Implementation may define type checking only as specified by accepted ADR-0027'

require_text "$decision" 'Decision: approved'
require_text "$decision" 'ADR-0027 is accepted'
require_text "$decision" 'M0018 ambiguity is resolved'

require_text "$task" 'Status: `(review|complete)`'

require_absent_text "$source" 'check_expression|check_declaration|infer_type|literal_type|resolve_call|check_assignment|TypedExpression|TypedProgram|WellTyped'
require_absent_text crates/newlang/src/lib.rs 'pub mod hir|pub mod mir|pub mod backend'

echo "m0018-accepted: type checking core accepted ADR validation passed"
