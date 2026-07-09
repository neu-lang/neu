#!/usr/bin/env sh
set -eu

fail() {
  echo "m0013-ast-shell: $*" >&2
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
task=docs/tasks/M0013-007-expression-statement-pattern-ast-shell.md
ast=crates/newlang/src/ast.rs
ast_tests=crates/newlang/tests/ast.rs

require_file "$adr"
require_file "$task"
require_file "$ast"
require_file "$ast_tests"

require_text "$adr" '^Status: Accepted$'
require_text "$task" 'Milestone: `M0013`'

for name in \
  Block \
  LiteralExpression \
  NameExpression \
  GroupedExpression \
  IfExpression \
  BinaryExpression \
  UnaryExpression \
  CallExpression \
  MemberExpression \
  VariableDeclarationStatement \
  AssignmentStatement \
  ReturnStatement \
  ExpressionStatement \
  WildcardPattern \
  LiteralPattern \
  BindingPattern \
  QualifiedCasePattern \
  GroupedPattern
do
  require_text "$ast" "$name"
done

require_text "$ast" 'add_block'
require_text "$ast" 'add_literal_expression'
require_text "$ast" 'add_name_expression'
require_text "$ast" 'add_grouped_expression'
require_text "$ast" 'add_if_expression'
require_text "$ast" 'add_binary_expression'
require_text "$ast" 'add_unary_expression'
require_text "$ast" 'add_call_expression'
require_text "$ast" 'add_member_expression'
require_text "$ast" 'add_variable_declaration_statement'
require_text "$ast" 'add_assignment_statement'
require_text "$ast" 'add_return_statement'
require_text "$ast" 'add_expression_statement'
require_text "$ast" 'add_wildcard_pattern'
require_text "$ast" 'add_literal_pattern'
require_text "$ast" 'add_binding_pattern'
require_text "$ast" 'add_qualified_case_pattern'
require_text "$ast" 'add_grouped_pattern'

require_text "$ast_tests" 'expression_statement_pattern_shell_nodes_preserve_kind_and_span'
require_text "$ast_tests" 'AstNodeKind::Block'
require_text "$ast_tests" 'AstNodeKind::LiteralExpression'
require_text "$ast_tests" 'AstNodeKind::VariableDeclarationStatement'
require_text "$ast_tests" 'AstNodeKind::QualifiedCasePattern'

require_text "$task" 'Status: `complete`'

require_absent_text crates/newlang/src/parser.rs 'parse_when|parse_match|parse_coroutine|parse_unsafe'
require_absent_text "$ast" 'TypeId|ResolvedType|SymbolId|Constraint|CapabilitySet|OwnershipState|BorrowState|FlowFact|Exhaustive|CoroutineFrame|UnsafeBoundary|Hir|Mir'

echo "m0013-ast-shell: expression, statement, and pattern AST shell validation passed"
