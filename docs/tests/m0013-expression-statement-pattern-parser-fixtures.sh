#!/usr/bin/env sh
set -eu

fail() {
  echo "m0013-fixtures: $*" >&2
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
task=docs/tasks/M0013-006-expression-statement-pattern-parser-fixtures.md
milestone=docs/milestones/M0013-expression-statement-and-pattern-parser.md

require_file "$adr"
require_file "$task"
require_file "$milestone"
require_text "$adr" '^Status: Accepted$'
require_text "$task" 'Milestone: `M0013`'

for fixture in \
  tests/fixtures/parser/expressions/positive.fixture.toml \
  tests/fixtures/parser/expressions/negative.fixture.toml \
  tests/fixtures/parser/expressions/diagnostics.fixture.toml \
  tests/fixtures/parser/statements/positive.fixture.toml \
  tests/fixtures/parser/statements/negative.fixture.toml \
  tests/fixtures/parser/statements/diagnostics.fixture.toml \
  tests/fixtures/parser/patterns/positive.fixture.toml \
  tests/fixtures/parser/patterns/negative.fixture.toml \
  tests/fixtures/parser/patterns/diagnostics.fixture.toml
do
  require_file "$fixture"
  require_text "$fixture" '^kind = "parser-expression-statement-pattern-fixture"$'
  require_text "$fixture" '^milestone = "M0013"$'
  require_text "$fixture" '^source = "docs/adr/ADR-0024-expression-statement-pattern-syntax.md"$'
  require_absent_text "$fixture" 'Kotlin|Rust|Go'
  require_absent_text "$fixture" 'type_check|flow_typing|ownership|borrow_check|exhaustiveness|coroutine_semantics|unsafe_semantics|hir|mir|backend'
done

require_text tests/fixtures/parser/expressions/positive.fixture.toml 'name = "literal_name_grouped_and_precedence_expressions"'
require_text tests/fixtures/parser/expressions/positive.fixture.toml 'source_text = "1 \+ 2 \* 3 \(foo\.bar\)"'
require_text tests/fixtures/parser/expressions/positive.fixture.toml 'expected_forms = \["binary-expression", "grouped-expression", "member-expression"\]'
require_text tests/fixtures/parser/expressions/positive.fixture.toml 'name = "call_member_and_if_expression"'
require_text tests/fixtures/parser/expressions/positive.fixture.toml 'source_text = "if \(ready\) \{ service\.run\(arg, 2\); \} else \{ fallback\(\); \}"'

require_text tests/fixtures/parser/statements/positive.fixture.toml 'name = "local_bindings_assignment_return_and_expression_statement"'
require_text tests/fixtures/parser/statements/positive.fixture.toml 'source_text = "val answer: Int = compute\(\); var next = answer; next = next \+ 1; log\(next\); return next;"'
require_text tests/fixtures/parser/statements/positive.fixture.toml 'expected_forms = \["variable-declaration", "assignment-statement", "expression-statement", "return-statement"\]'
require_text tests/fixtures/parser/statements/positive.fixture.toml 'name = "block_with_trailing_expression"'
require_text tests/fixtures/parser/statements/positive.fixture.toml 'source_text = "\{ val value = compute\(\); value \}"'

require_text tests/fixtures/parser/patterns/positive.fixture.toml 'name = "wildcard_literal_binding_and_grouped_patterns"'
require_text tests/fixtures/parser/patterns/positive.fixture.toml 'source_text = "_ 0 true null name \(name\)"'
require_text tests/fixtures/parser/patterns/positive.fixture.toml 'name = "qualified_case_pattern_with_nested_arguments"'
require_text tests/fixtures/parser/patterns/positive.fixture.toml 'source_text = "Result\.Ok\(value\) demo\.Option\.Some\(Pair\(left, _\)\)"'

require_text tests/fixtures/parser/expressions/negative.fixture.toml 'name = "assignment_is_not_expression"'
require_text tests/fixtures/parser/expressions/negative.fixture.toml 'unsupported_expression_form'
require_text tests/fixtures/parser/expressions/negative.fixture.toml 'name = "malformed_call_and_member_access"'
require_text tests/fixtures/parser/expressions/negative.fixture.toml 'malformed_call_expression'
require_text tests/fixtures/parser/expressions/negative.fixture.toml 'malformed_member_access'

require_text tests/fixtures/parser/statements/negative.fixture.toml 'name = "malformed_variable_declaration_and_assignment"'
require_text tests/fixtures/parser/statements/negative.fixture.toml 'malformed_variable_declaration'
require_text tests/fixtures/parser/statements/negative.fixture.toml 'malformed_assignment'
require_text tests/fixtures/parser/statements/negative.fixture.toml 'name = "deferred_loop_coroutine_and_unsafe_forms"'
require_text tests/fixtures/parser/statements/negative.fixture.toml 'unsupported_statement_form'

require_text tests/fixtures/parser/patterns/negative.fixture.toml 'name = "unsupported_match_or_when_pattern_context"'
require_text tests/fixtures/parser/patterns/negative.fixture.toml 'unsupported_pattern_form'
require_text tests/fixtures/parser/patterns/negative.fixture.toml 'name = "malformed_pattern_arguments"'
require_text tests/fixtures/parser/patterns/negative.fixture.toml 'malformed_pattern'

require_text tests/fixtures/parser/expressions/diagnostics.fixture.toml 'missing_expression'
require_text tests/fixtures/parser/expressions/diagnostics.fixture.toml 'unexpected_token_in_expression'
require_text tests/fixtures/parser/expressions/diagnostics.fixture.toml 'unsupported_expression_form'
require_text tests/fixtures/parser/expressions/diagnostics.fixture.toml 'malformed_binary_expression'
require_text tests/fixtures/parser/expressions/diagnostics.fixture.toml 'malformed_call_expression'
require_text tests/fixtures/parser/expressions/diagnostics.fixture.toml 'malformed_member_access'
require_text tests/fixtures/parser/expressions/diagnostics.fixture.toml 'malformed_conditional'

require_text tests/fixtures/parser/statements/diagnostics.fixture.toml 'malformed_block'
require_text tests/fixtures/parser/statements/diagnostics.fixture.toml 'missing_statement'
require_text tests/fixtures/parser/statements/diagnostics.fixture.toml 'unexpected_token_in_statement'
require_text tests/fixtures/parser/statements/diagnostics.fixture.toml 'unsupported_statement_form'
require_text tests/fixtures/parser/statements/diagnostics.fixture.toml 'malformed_variable_declaration'
require_text tests/fixtures/parser/statements/diagnostics.fixture.toml 'malformed_assignment'
require_text tests/fixtures/parser/statements/diagnostics.fixture.toml 'malformed_return_statement'
require_text tests/fixtures/parser/statements/diagnostics.fixture.toml 'malformed_unsafe_block'
require_text tests/fixtures/parser/statements/diagnostics.fixture.toml 'malformed_coroutine_construct'

require_text tests/fixtures/parser/patterns/diagnostics.fixture.toml 'malformed_pattern'
require_text tests/fixtures/parser/patterns/diagnostics.fixture.toml 'unsupported_pattern_form'
require_text tests/fixtures/parser/patterns/diagnostics.fixture.toml 'missing_pattern_arm_body'
require_text tests/fixtures/parser/patterns/diagnostics.fixture.toml 'pattern recovery boundary'

require_text "$milestone" 'Expression fixtures pass'
require_text "$milestone" 'Statement fixtures pass'
require_text "$milestone" 'Pattern fixtures pass'

echo "m0013-fixtures: expression, statement, and pattern parser fixture validation passed"
