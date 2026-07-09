#!/usr/bin/env sh
set -eu

fail() {
  echo "m0012-concrete-draft: $*" >&2
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

proposal=docs/adr/proposals/ADR-0023-type-and-generic-syntax.md
ambiguity=docs/ambiguities/M0008-type-generic-syntax.md
decision=docs/adr/proposals/reviews/ADR-0023-chief-architect-decision.md
task=docs/tasks/M0012-004-type-generic-syntax-concrete-draft.md

require_file "$proposal"
require_file "$ambiguity"
require_file "$decision"
require_file "$task"

require_text "$proposal" '^Status: Draft proposal - not accepted source of truth$'
require_text "$proposal" '^## Concrete Draft Grammar$'
require_text "$proposal" '^### Type Grammar Overview$'
require_text "$proposal" '^### Named Type References$'
require_text "$proposal" '^### Nullable Type Syntax$'
require_text "$proposal" '^### Generic Parameter Syntax$'
require_text "$proposal" '^### Generic Argument Syntax$'
require_text "$proposal" '^### Capability-Bound Syntax$'
require_text "$proposal" '^### Function Type Syntax$'
require_text "$proposal" '^### Type Grouping And Binding$'
require_text "$proposal" '^### Recovery Boundaries$'
require_text "$proposal" '^### Type Syntax Diagnostics$'
require_text "$proposal" '^### Review Attack Cases$'
require_text "$proposal" '^### Concrete Deferrals$'
require_text "$proposal" 'type = nullable-type'
require_text "$proposal" 'nullable-type = primary-type `\?`\?'
require_text "$proposal" 'named-type = qualified-name generic-arguments\?'
require_text "$proposal" 'generic-parameters = `<` generic-parameter \(`,` generic-parameter\)\* `>`'
require_text "$proposal" 'generic-arguments = `<` type \(`,` type\)\* `>`'
require_text "$proposal" 'capability-bound-list = capability-bound \(`&` capability-bound\)\*'
require_text "$proposal" 'function-type = `\(` function-type-parameters\? `\)` `->` type'
require_text "$proposal" 'Box<T\?>\?'
require_text "$proposal" 'fun f<T: Send & Share>\(\);'
require_text "$proposal" '\(T\) -> U\?'
require_text "$proposal" '\(\(T\) -> U\)\?'
require_text "$proposal" 'missing_type_name'
require_text "$proposal" 'malformed_capability_bound'
require_text "$proposal" 'primary span'
require_text "$proposal" 'recovery action'

require_text "$ambiguity" 'Status: `resolved`'
require_text "$ambiguity" 'docs/adr/ADR-0023-type-and-generic-syntax.md'
require_text "$decision" '^Decision: approved$'
require_text "$decision" 'Accepted source of truth: `docs/adr/ADR-0023-type-and-generic-syntax.md`'
require_text "$task" 'Status: `complete`'

require_file docs/adr/ADR-0023-type-and-generic-syntax.md
require_text docs/adr/ADR-0023-type-and-generic-syntax.md '^Status: Accepted$'
require_text docs/SPEC.md '^## ADR-0023: Type And Generic Syntax$'
require_absent_text crates/newlang/src/parser.rs 'parse_type|parse_generic|parse_capability|TypeRef|GenericParameter|GenericArgument|CapabilityBound|NullableType|FunctionType'

echo "m0012-concrete-draft: concrete draft history and accepted ADR validation passed"
