#!/usr/bin/env sh
set -eu

fail() {
  echo "m0018-type-checking-ambiguity-blocker: $*" >&2
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

task=docs/tasks/M0018-001-type-checking-ambiguity-blocker.md
milestone=docs/milestones/M0018-type-checking-core.md
ambiguity=docs/ambiguities/M0018-type-checking-core.md
source=crates/compiler/src/type_check.rs
lib=crates/compiler/src/lib.rs
test_file=crates/compiler/tests/type_check.rs

require_file "$task"
require_file "$milestone"
require_file "$ambiguity"
require_file "$source"
require_file "$test_file"

require_text "$task" 'Milestone: `M0018`'
require_text "$milestone" '- \[x\] Ambiguous type rules are blocked\.'
require_text "$ambiguity" 'Literal typing and overload rules may be unspecified'
require_text "$ambiguity" 'No implementation may proceed on literal typing'
require_text "$lib" 'pub mod type_check;'
require_text "$source" 'pub enum AmbiguousTypeRule'
require_text "$source" 'LiteralTyping'
require_text "$source" 'PrimitiveScalarCatalog'
require_text "$source" 'AssignmentCompatibility'
require_text "$source" 'CallResolution'
require_text "$source" 'FunctionTypeApplication'
require_text "$source" 'pub enum TypeCheckDiagnosticKind'
require_text "$source" 'AmbiguousTypeRule'
require_text "$source" 'pub struct TypeCheckDiagnostic'
require_text "$source" 'pub fn ambiguous_type_rule'
require_text "$source" 'pub struct TypeCheckReport'
require_text "$source" 'pub fn blocked'
require_text "$test_file" 'ambiguous_type_rule_diagnostic_preserves_rule_and_node'
require_text "$test_file" 'ambiguous_type_rules_cover_m0018_blockers'
require_text "$test_file" 'type_check_report_records_blockers_without_successful_output'

require_absent_text "$source" 'check_expression|check_declaration|infer_type|literal_type|resolve_call|check_assignment|TypedExpression|TypedProgram|WellTyped'
require_absent_text "$source" 'Ownership|Borrow|FlowType|Hir|Mir|Cranelift|LLVM'

cargo test -p compiler --test type_check

echo "m0018-type-checking-ambiguity-blocker: type checking ambiguity blocker validation passed"
