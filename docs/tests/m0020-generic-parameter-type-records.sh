#!/usr/bin/env sh
set -eu

fail() {
  echo "m0020-generic-parameter-type-records: $*" >&2
  exit 1
}

require_text() {
  grep -Eq -- "$2" "$1" || fail "missing expected pattern in $1: $2"
}

source=crates/compiler/src/type_check.rs
tests=crates/compiler/tests/type_check.rs

[ -f "$source" ] || fail "missing type-check source"
[ -f "$tests" ] || fail "missing type-check tests"

require_text "$source" 'pub struct GenericParameterTypeRecord'
require_text "$source" 'pub fn build_m0020_generic_parameter_types'
require_text "$source" 'GenericParameterType::new'
require_text "$tests" 'm0020_generic_parameter_types_preserve_parameter_identity_and_source_order'

echo "m0020-generic-parameter-type-records: type record contract validated"
