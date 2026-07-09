#!/usr/bin/env sh
set -eu

fail() {
  echo "m0011-parser-impl: $*" >&2
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

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope path exists during declaration parser task: $1"
}

require_absent_text() {
  file="$1"
  pattern="$2"
  if grep -Eq "$pattern" "$file"; then
    fail "unexpected pattern in $file: $pattern"
  fi
}

require_file crates/newlang/src/parser.rs
require_file crates/newlang/tests/parser.rs
require_file docs/tasks/M0011-008-declaration-parser-implementation.md
require_file docs/adr/ADR-0022-declaration-syntax.md

require_text crates/newlang/src/lib.rs '^pub mod parser;$'
require_text crates/newlang/src/parser.rs 'pub fn parse_source'
require_text crates/newlang/src/parser.rs 'MisplacedPackageDeclaration'
require_text crates/newlang/src/parser.rs 'MisplacedImportDeclaration'
require_text crates/newlang/src/parser.rs 'DuplicateVisibilityModifier'
require_text crates/newlang/src/parser.rs 'UnsupportedDeclarationModifier'
require_text crates/newlang/src/parser.rs 'MissingDeclarationName'
require_text crates/newlang/src/parser.rs 'MalformedDeclarationHeader'
require_text crates/newlang/src/parser.rs 'InvalidMemberDeclarationPosition'
require_text crates/newlang/src/parser.rs 'UnexpectedTokenInDeclarationBody'
require_text crates/newlang/tests/parser.rs 'parses_package_import_and_function_declaration'
require_text crates/newlang/tests/parser.rs 'parses_nested_declaration_body_shells'
require_text crates/newlang/tests/parser.rs 'reports_misplaced_package_and_import'
require_text crates/newlang/tests/parser.rs 'rejects_deferred_expression_and_field_syntax'
require_text docs/tasks/M0011-008-declaration-parser-implementation.md 'Status: `complete`'

require_absent_text crates/newlang/src/parser.rs 'TypeRef|Symbol|NameResolution|Hir|Mir|Borrow|Ownership|FlowFact|OwnershipState|BorrowState'
require_absent_path crates/newlang/src/hir.rs
require_absent_path crates/newlang/src/mir.rs

echo "m0011-parser-impl: declaration parser implementation validation passed"
