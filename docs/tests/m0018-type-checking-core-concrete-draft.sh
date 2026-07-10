#!/usr/bin/env sh
set -eu

fail() {
  echo "m0018-concrete-draft: $*" >&2
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
  grep -Eq -- "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

require_absent_text() {
  file="$1"
  pattern="$2"
  if grep -Eq -- "$pattern" "$file"; then
    fail "unexpected pattern in $file: $pattern"
  fi
}

proposal=docs/adr/proposals/ADR-0027-type-checking-core.md
accepted=docs/adr/ADR-0027-type-checking-core.md
ambiguity=docs/ambiguities/M0018-type-checking-core.md
task=docs/tasks/M0018-004-type-checking-core-concrete-draft.md
source=crates/newlang/src/type_check.rs

require_file "$proposal"
require_file "$accepted"
require_file "$ambiguity"
require_file "$task"
require_file "$source"

require_text "$proposal" '^Status: Draft proposal - not accepted source of truth$'
require_text "$proposal" '^## Draft Concrete Type Checking Model$'
require_text "$proposal" '^## Draft Typed Output Shape$'
require_text "$proposal" '^## Draft Primitive Type Identity$'
require_text "$proposal" '^## Draft Included Expression Forms$'
require_text "$proposal" '^## Draft Assignment Compatibility$'
require_text "$proposal" '^## Draft Direct Call Deferral$'
require_text "$proposal" '^## Draft Function Type Application Deferral$'
require_text "$proposal" '^## Draft Type Checking Diagnostics$'
require_text "$proposal" '^## Draft Unsupported And Ambiguous Rules$'
require_text "$proposal" '^## Explicit Draft Deferrals$'

require_text "$proposal" 'expression type table'
require_text "$proposal" 'declaration signature table'
require_text "$proposal" 'diagnostics list'
require_text "$proposal" 'no typed AST rewrite'
require_text "$proposal" 'PrimitiveType'
require_text "$proposal" 'Bool'
require_text "$proposal" 'Int'
require_text "$proposal" 'String'
require_text "$proposal" 'Unit'
require_text "$proposal" 'Null'
require_text "$proposal" 'type-checking identity only'
require_text "$proposal" 'no ABI or layout meaning'
require_text "$proposal" 'Non-null base values are assignment-compatible with their nullable wrapper'
require_text "$proposal" '`Null` is assignment-compatible only with nullable target types'
require_text "$proposal" 'Direct function declaration calls are deferred for M0018'
require_text "$proposal" 'Structural function type application is deferred for M0018'
require_text "$proposal" 'type_mismatch'
require_text "$proposal" 'unresolved_type_rule'
require_text "$proposal" 'unsupported_type_rule'
require_text "$proposal" 'ambiguous_type_rule'
require_text "$proposal" 'Primary span:'
require_text "$proposal" 'Recovery action:'
require_text "$proposal" 'Source-of-truth citation:'
require_text "$proposal" 'Safe suggestion policy:'
require_text "$proposal" 'stable rule identifier'

require_text "$proposal" 'overload resolution'
require_text "$proposal" 'implicit numeric conversion'
require_text "$proposal" 'generic constraint solving'
require_text "$proposal" 'member lookup'
require_text "$proposal" 'ownership and move analysis'
require_text "$proposal" 'borrow checking'
require_text "$proposal" 'HIR lowering'
require_text "$proposal" 'MIR lowering'
require_text "$proposal" 'backend code generation'

require_text "$accepted" '^Status: Accepted$'
require_text "$ambiguity" 'Status: `resolved`'
require_text "$task" 'Status: `(review|complete)`'
require_text docs/SPEC.md '^## ADR-0027: Type Checking Core$'
require_absent_text "$source" 'check_expression|check_declaration|infer_type|literal_type|resolve_call|check_assignment|TypedExpression|TypedProgram|WellTyped'

echo "m0018-concrete-draft: concrete type checking draft validation passed"
