#!/usr/bin/env sh
set -eu

fail() {
  echo "m0019-concrete-draft: $*" >&2
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

proposal=docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md
accepted=docs/adr/ADR-0028-nullability-and-flow-typing.md
ambiguity=docs/ambiguities/M0019-nullability-and-flow-typing.md
task=docs/tasks/M0019-004-nullability-flow-concrete-draft.md
source=crates/newlang/src/type_check.rs

require_file "$proposal"
require_file "$accepted"
require_file "$ambiguity"
require_file "$task"
require_file "$source"

require_text "$proposal" '^Status: Draft proposal - not accepted source of truth$'
require_text "$proposal" '^## Draft Concrete Nullability And Flow Model$'
require_text "$proposal" '^## Draft Null-Test Recognition$'
require_text "$proposal" '^## Draft Branch Region Boundaries$'
require_text "$proposal" '^## Draft Refined Output Shape$'
require_text "$proposal" '^## Draft Shadowing And Nested Scope Rules$'
require_text "$proposal" '^## Draft Nullable Use Rules$'
require_text "$proposal" '^## Draft Smart-Cast Eligibility$'
require_text "$proposal" '^## Draft Mutation Invalidation$'
require_text "$proposal" '^## Draft Flow Diagnostics$'
require_text "$proposal" '^## Explicit Draft Deferrals$'

require_text "$proposal" 'flow-specific condition recognizer'
require_text "$proposal" 'does not require general binary expression type checking'
require_text "$proposal" 'then branch'
require_text "$proposal" 'else branch'
require_text "$proposal" 'optional trailing expression'
require_text "$proposal" 'refinement table'
require_text "$proposal" 'refined expression type entries'
require_text "$proposal" 'original nullable type of the binding'
require_text "$proposal" 'shadowing'
require_text "$proposal" 'nested block'
require_text "$proposal" 'duplicate local bindings'
require_text "$proposal" 'ambiguous local binding'

require_text "$proposal" 'invalid_nullable_use'
require_text "$proposal" 'invalidated_refinement'
require_text "$proposal" 'unsupported_flow_rule'
require_text "$proposal" 'ambiguous_flow_rule'
require_text "$proposal" 'Primary span:'
require_text "$proposal" 'Recovery action:'
require_text "$proposal" 'Source-of-truth citation:'
require_text "$proposal" 'Safe suggestion policy:'
require_text "$proposal" 'stable rule identifier'
require_text "$proposal" 'mutable_local_refinement_deferred'
require_text "$proposal" 'boolean_combination_refinement_deferred'
require_text "$proposal" 'member_refinement_deferred'
require_text "$proposal" 'call_result_refinement_deferred'
require_text "$proposal" 'exclusive_borrow_refinement_deferred'

require_text "$proposal" 'member access on nullable receivers'
require_text "$proposal" 'function call effects'
require_text "$proposal" 'alias analysis'
require_text "$proposal" 'coroutine suspension effects'
require_text "$proposal" 'unsafe and FFI nullability'
require_text "$proposal" 'generic nullable constraints'
require_text "$proposal" 'pattern-based refinement'
require_text "$proposal" 'HIR lowering'
require_text "$proposal" 'MIR lowering'
require_text "$proposal" 'backend code generation'

require_text "$accepted" '^Status: Accepted$'
require_text "$ambiguity" 'Status: `resolved`'
require_text "$task" 'Status: `(in_progress|review|complete)`'
require_text docs/SPEC.md '^## ADR-0028: Nullability And Flow Typing$'
require_absent_text "$source" 'FlowRefinement|SmartCast|invalid_nullable_use|invalidated_refinement|unsupported_flow_rule|ambiguous_flow_rule'

echo "m0019-concrete-draft: concrete nullability and flow typing draft validation passed"
