#!/usr/bin/env sh
set -eu

fail() {
  echo "m0019-accepted: $*" >&2
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

adr=docs/adr/ADR-0028-nullability-and-flow-typing.md
spec=docs/SPEC.md
ambiguity=docs/ambiguities/M0019-nullability-and-flow-typing.md
decision=docs/adr/proposals/reviews/ADR-0028-chief-architect-decision.md
task=docs/tasks/M0019-005-accept-nullability-flow-adr.md
source=crates/compiler/src/type_check.rs

require_file "$adr"
require_file "$spec"
require_file "$ambiguity"
require_file "$decision"
require_file "$task"
require_file "$source"

require_text "$adr" '^# ADR-0028: Nullability And Flow Typing$'
require_text "$adr" '^Status: Accepted$'
require_text "$adr" '^## Decision$'
require_text "$adr" 'narrow M0019 nullability and flow-typing subset'
require_text "$adr" '^## Concrete Nullability And Flow Model$'
require_text "$adr" '^## Null-Test Recognition$'
require_text "$adr" '^## Branch Region Boundaries$'
require_text "$adr" '^## Refined Output Shape$'
require_text "$adr" '^## Shadowing And Nested Scope Rules$'
require_text "$adr" '^## Nullable Use Rules$'
require_text "$adr" '^## Smart-Cast Eligibility$'
require_text "$adr" '^## Mutation Invalidation$'
require_text "$adr" '^## Flow Diagnostics$'
require_text "$adr" '^## Explicit Deferrals$'
require_text "$adr" 'flow-specific condition recognizer'
require_text "$adr" 'does not require general binary expression type checking'
require_text "$adr" 'then branch'
require_text "$adr" 'else branch'
require_text "$adr" 'optional trailing expression'
require_text "$adr" 'refinement table'
require_text "$adr" 'refined expression type entries'
require_text "$adr" 'original nullable type of the binding'
require_text "$adr" 'Shadowing rules'
require_text "$adr" 'duplicate local bindings'
require_text "$adr" 'ambiguous local binding'
require_text "$adr" 'Diagnostic: `invalid_nullable_use`'
require_text "$adr" 'Diagnostic: `invalidated_refinement`'
require_text "$adr" 'Diagnostic: `unsupported_flow_rule`'
require_text "$adr" 'Diagnostic: `ambiguous_flow_rule`'
require_text "$adr" 'Primary span:'
require_text "$adr" 'Recovery action:'
require_text "$adr" 'Source-of-truth citation:'
require_text "$adr" 'Safe suggestion policy:'
require_text "$adr" 'stable rule identifier'
require_text "$adr" 'mutable_local_refinement_deferred'
require_text "$adr" 'boolean_combination_refinement_deferred'
require_text "$adr" 'member_refinement_deferred'
require_text "$adr" 'call_result_refinement_deferred'
require_text "$adr" 'exclusive_borrow_refinement_deferred'
require_text "$adr" 'member access on nullable receivers'
require_text "$adr" 'function call effects'
require_text "$adr" 'alias analysis'
require_text "$adr" 'coroutine suspension effects'
require_text "$adr" 'unsafe and FFI nullability'
require_text "$adr" 'generic nullable constraints'
require_text "$adr" 'pattern-based refinement'
require_text "$adr" 'HIR lowering'
require_text "$adr" 'MIR lowering'
require_text "$adr" 'backend code generation'
require_absent_text "$adr" '^Status: Draft proposal'
require_absent_text "$adr" '^## Non-Authority Notice$'
require_absent_text "$adr" 'not accepted source of truth'
require_absent_text "$adr" 'This section is a draft direction'

require_text "$spec" '^## ADR-0028: Nullability And Flow Typing$'
require_text "$spec" 'M0019 defines a narrow nullability and flow-typing subset'
require_text "$spec" 'Null-test recognition is a flow-specific condition recognizer'
require_text "$spec" 'Refined output remains side-table metadata'
require_text "$spec" 'Flow diagnostics include `invalid_nullable_use`, `invalidated_refinement`, `unsupported_flow_rule`, and `ambiguous_flow_rule`'

require_text "$ambiguity" 'Status: `resolved`'
require_text "$ambiguity" 'Resolution Source: `docs/adr/ADR-0028-nullability-and-flow-typing.md`'
require_text "$ambiguity" 'Resolved Date: `2026-07-10`'
require_text "$ambiguity" 'Implementation may define nullability and flow typing only as specified by accepted ADR-0028'

require_text "$decision" 'Decision: approved'
require_text "$decision" 'ADR-0028 is accepted'
require_text "$decision" 'M0019 ambiguity is resolved'

require_text "$task" 'Status: `(in_progress|review|complete)`'

require_absent_text "$source" 'recognize_null_test|apply_smart_cast|check_nullable_use|walk_if_branch'
require_absent_text crates/compiler/src/lib.rs 'pub mod hir|pub mod mir|pub mod backend'

echo "m0019-accepted: nullability and flow typing accepted ADR validation passed"
