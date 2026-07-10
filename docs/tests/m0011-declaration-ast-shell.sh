#!/usr/bin/env sh
set -eu

fail() {
  echo "m0011-ast-shell: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope path exists during declaration AST shell task: $1"
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

ast=crates/compiler/src/ast.rs
tests=crates/compiler/tests/ast.rs
doc=docs/ast/data-model.md
task=docs/tasks/M0011-007-declaration-ast-shell.md

require_file "$ast"
require_file "$tests"
require_file "$doc"
require_file "$task"
require_file docs/adr/ADR-0022-declaration-syntax.md

require_text "$ast" 'PackageDeclaration'
require_text "$ast" 'ImportDeclaration'
require_text "$ast" 'FunctionDeclaration'
require_text "$ast" 'StructDeclaration'
require_text "$ast" 'EnumDeclaration'
require_text "$ast" 'InterfaceDeclaration'
require_text "$ast" 'DeclarationBody'
require_text "$ast" 'add_package_declaration'
require_text "$ast" 'add_import_declaration'
require_text "$ast" 'add_function_declaration'
require_text "$ast" 'add_struct_declaration'
require_text "$ast" 'add_enum_declaration'
require_text "$ast" 'add_interface_declaration'
require_text "$ast" 'add_declaration_body'

require_text "$tests" 'declaration_shell_nodes_preserve_kind_and_span'
require_text "$tests" 'declaration_body_node_is_syntax_only'
require_text "$doc" 'Status: M0013 expression statement and pattern AST shell'
require_text "$doc" 'ADR-0022 declaration node kinds'
require_text "$task" 'Status: `complete`'

require_absent_text "$ast" 'TypeRef|Resolved|Symbol|Borrow|Ownership|NameResolution|Hir|Mir|FlowFact|Exhaustive|CoroutineFrame|UnsafeBoundary'
require_absent_path crates/compiler/src/hir.rs
require_absent_path crates/compiler/src/mir.rs

echo "m0011-ast-shell: declaration AST shell validation passed"
