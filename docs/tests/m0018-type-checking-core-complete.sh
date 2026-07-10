#!/usr/bin/env sh
set -eu

fail() {
  echo "m0018-complete: $*" >&2
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

milestone=docs/milestones/M0018-type-checking-core.md
source=crates/newlang/src/type_check.rs
tests=crates/newlang/tests/type_check.rs

require_file "$milestone"
require_file "$source"
require_file "$tests"
require_file docs/tasks/M0018-023-type-check-core-orchestration.md
require_file docs/tasks/reviews/M0018-023-review.md
require_file docs/tasks/soundness/M0018-023-soundness.md

require_text "$milestone" '^- \[x\] Well-typed fixtures pass\.$'
require_text "$milestone" '^- \[x\] Ill-typed fixtures diagnose\.$'
require_text "$milestone" '^- \[x\] Ambiguous type rules are blocked\.$'
require_absent_text "$milestone" '^- \[ \] Well-typed fixtures pass\.$'
require_absent_text "$milestone" '^- \[ \] Ill-typed fixtures diagnose\.$'

require_text "$source" 'pub fn type_m0018_core'
require_text "$source" 'TypeRuleDiagnostic::MissingAnnotationType'
require_text "$source" 'TypeRuleDiagnostic::MissingResolvedNameType'
require_text "$source" 'TypeRuleDiagnostic::DirectCallDeferred'
require_text "$source" 'TypeRuleDiagnostic::MemberExpressionDeferred'
require_text "$source" 'TypeRuleDiagnostic::BinaryExpressionDeferred'
require_text "$source" 'TypeRuleDiagnostic::UnaryExpressionDeferred'
require_text "$source" 'TypeRuleDiagnostic::IfValueDeferred'

require_text "$tests" 'fn m0018_core_types_well_typed_accepted_fixture'
require_text "$tests" 'fn m0018_core_reports_mismatch_unresolved_and_unsupported_diagnostics'
require_text "$tests" 'TypeCheckDiagnosticKind::TypeMismatch'
require_text "$tests" 'TypeCheckDiagnosticKind::UnsupportedTypeRule'
require_text "$tests" 'TypeCheckDiagnosticKind::UnresolvedTypeRule'
require_text "$tests" 'AmbiguousTypeRule::CallResolution'

echo "m0018-complete: type checking core milestone validation passed"
