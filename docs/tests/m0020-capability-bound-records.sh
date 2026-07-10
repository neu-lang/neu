#!/usr/bin/env sh
set -eu

fail() {
  echo "m0020-capability-bound-records: $*" >&2
  exit 1
}

require_text() {
  grep -Eq -- "$2" "$1" || fail "missing expected pattern in $1: $2"
}

source=crates/newlang/src/type_check.rs
tests=crates/newlang/tests/type_check.rs

[ -f "$source" ] || fail "missing type-check source"
[ -f "$tests" ] || fail "missing type-check tests"

require_text "$source" 'pub struct CapabilityBoundRecord'
require_text "$source" 'pub fn build_m0020_capability_bound_records'
require_text "$tests" 'm0020_capability_bound_records_preserve_occurrences_without_interpretation'

echo "m0020-capability-bound-records: capability record contract validated"
