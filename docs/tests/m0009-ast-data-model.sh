#!/usr/bin/env sh
set -eu

fail() {
  echo "m0009-ast: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope path exists during M0009 AST shell: $1"
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

require_file crates/newlang/src/ast.rs
require_file crates/newlang/tests/ast.rs
require_file docs/ast/data-model.md
require_file docs/syntax/grammar-authority-ledger.md
require_file docs/tasks/M0009-001-ast-span-shell.md

require_text crates/newlang/src/lib.rs '^pub mod ast;$'
require_text crates/newlang/src/ast.rs 'pub struct AstNodeId'
require_text crates/newlang/src/ast.rs 'pub enum AstNodeKind'
require_text crates/newlang/src/ast.rs 'SourceFile'
require_text crates/newlang/src/ast.rs 'ByteSpan'
require_text crates/newlang/src/ast.rs 'pub struct AstArena'
require_text crates/newlang/tests/ast.rs 'source_file_root_node_preserves_span'
require_text docs/ast/data-model.md '^# AST Data Model$'
require_text docs/ast/data-model.md 'Status: M0011 declaration AST shell'
require_text docs/ast/data-model.md 'The accepted M0009 root node kind is `SourceFile`'
require_text docs/ast/data-model.md 'Type, expression, statement, and pattern nodes remain deferred'
require_text docs/ast/data-model.md 'docs/syntax/grammar-authority-ledger.md'
require_text docs/milestones/M0009-ast-data-model.md '\[x\] AST model exists'
require_text docs/milestones/M0009-ast-data-model.md '\[x\] Span retention is tested'
require_text docs/milestones/M0009-ast-data-model.md '\[x\] Semantic analysis is not encoded in AST'

require_absent_text crates/newlang/src/ast.rs 'Expression|Statement|Pattern|TypeRef|Resolved|Symbol|Borrow|Ownership'
require_absent_path crates/newlang/src/parser.rs
require_absent_path crates/newlang/src/hir.rs
require_absent_path crates/newlang/src/mir.rs
require_absent_path crates/newlang/tests/parser.rs

echo "m0009-ast: AST data model validation passed"
