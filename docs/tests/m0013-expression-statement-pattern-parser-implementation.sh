#!/usr/bin/env sh
set -eu

fail() {
  echo "m0013-parser-impl: $*" >&2
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
task=docs/tasks/M0013-008-expression-statement-pattern-parser-implementation.md
parser=crates/newlang/src/parser.rs
parser_tests=crates/newlang/tests/parser.rs
milestone=docs/milestones/M0013-expression-statement-and-pattern-parser.md

require_file "$adr"
require_file "$task"
require_file "$parser"
require_file "$parser_tests"
require_file "$milestone"

require_text "$adr" '^Status: Accepted$'
require_text "$task" 'Milestone: `M0013`'

for diagnostic in \
  MissingExpression \
  UnexpectedTokenInExpression \
  UnsupportedExpressionForm \
  MalformedBinaryExpression \
  MalformedCallExpression \
  MalformedMemberAccess \
  MalformedBlock \
  MissingStatement \
  UnexpectedTokenInStatement \
  UnsupportedStatementForm \
  MalformedVariableDeclaration \
  MalformedAssignment \
  MalformedReturnStatement \
  MalformedConditional \
  MalformedPattern \
  UnsupportedPatternForm \
  MissingPatternArmBody \
  MalformedUnsafeBlock \
  MalformedCoroutineConstruct
do
  require_text "$parser" "$diagnostic"
done

require_text "$parser" 'fn parse_body_block'
require_text "$parser" 'fn parse_statement'
require_text "$parser" 'fn parse_expression'
require_text "$parser" 'fn parse_pattern'
require_text "$parser" 'add_block'
require_text "$parser" 'add_variable_declaration_statement'
require_text "$parser" 'add_assignment_statement'
require_text "$parser" 'add_return_statement'
require_text "$parser" 'add_expression_statement'
require_text "$parser" 'add_binary_expression'
require_text "$parser" 'add_call_expression'
require_text "$parser" 'add_member_expression'
require_text "$parser" 'add_if_expression'
require_text "$parser" 'add_qualified_case_pattern'

require_text "$parser_tests" 'parses_adr0024_body_statements_and_expressions'
require_text "$parser_tests" 'parses_trailing_expression_and_if_expression_body'
require_text "$parser_tests" 'reports_adr0024_body_diagnostics'
require_text "$parser_tests" 'rejects_deferred_body_forms'
require_text "$parser_tests" 'AstNodeKind::VariableDeclarationStatement'
require_text "$parser_tests" 'AstNodeKind::IfExpression'
require_text "$parser_tests" 'DiagnosticKind::MalformedVariableDeclaration'
require_text "$parser_tests" 'DiagnosticKind::MalformedUnsafeBlock'

require_text "$milestone" '\[x\] Parser implementation for approved body syntax exists'
require_text "$milestone" '\[x\] Invalid body syntax reports source-spanned diagnostics'
require_text "$milestone" '\[x\] Deferred body constructs remain rejected'

require_absent_text "$parser" 'type_check|flow_typing|ownership|borrow_check|exhaustiveness|lower_to_hir|emit_mir|backend|resolve_name|SymbolId|TypeId|OwnershipState|BorrowState|FlowFact'

echo "m0013-parser-impl: expression, statement, and pattern parser implementation validation passed"
