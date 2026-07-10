#!/usr/bin/env sh
set -eu

fail() {
  echo "m0021-enum-variant-parser-metadata: $*" >&2
  exit 1
}

require_text() {
  grep -Eq -- "$2" "$1" || fail "missing expected pattern in $1: $2"
}

source=crates/compiler/src/parser.rs
ast=crates/compiler/src/ast.rs
tests=crates/compiler/tests/parser.rs

[ -f "$source" ] || fail "missing parser source"
[ -f "$ast" ] || fail "missing AST source"
[ -f "$tests" ] || fail "missing parser tests"

require_text "$ast" 'EnumVariant'
require_text "$source" 'pub struct ParsedEnumVariant'
require_text "$source" 'fn parse_enum_body'
require_text "$tests" 'm0021_enum_variants_preserve_enclosing_enum_order_and_spans'
require_text "$tests" 'm0021_enum_variants_exclude_empty_and_payload_shaped_entries'

echo "m0021-enum-variant-parser-metadata: enum metadata contract validated"
