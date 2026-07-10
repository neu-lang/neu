#!/usr/bin/env sh
set -eu

fail() {
  echo "m0020-generic-parameter-metadata: $*" >&2
  exit 1
}

require_text() {
  grep -Eq -- "$2" "$1" || fail "missing expected pattern in $1: $2"
}

source=crates/compiler/src/parser.rs
tests=crates/compiler/tests/parser.rs

[ -f "$source" ] || fail "missing parser source"
[ -f "$tests" ] || fail "missing parser tests"

require_text "$source" 'pub struct ParsedGenericParameter'
require_text "$source" 'pub struct ParsedCapabilityBound'
require_text "$source" 'pub generic_parameters: Vec<ParsedGenericParameter>'
require_text "$tests" 'm0020_generic_parameter_metadata_preserves_parameters_and_capability_bounds'
require_text "$tests" 'm0020_generic_parameter_metadata_excludes_malformed_lists_and_arguments'

echo "m0020-generic-parameter-metadata: parser metadata contract validated"
