#!/usr/bin/env sh
set -eu

fail() {
  echo "m0019-local-binding-resolution-identity: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq -- "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

source=crates/compiler/src/name_resolution.rs
tests=crates/compiler/tests/name_resolution.rs
task=docs/tasks/M0019-011-local-binding-resolution-identity.md

require_file "$source"
require_file "$tests"
require_file "$task"

require_text "$source" 'pub struct ResolvedLocalBinding'
require_text "$source" 'reference: AstNodeId'
require_text "$source" 'binding: LocalBinding'
require_text "$source" 'pub fn resolved_local_bindings'
require_text "$source" 'ResolvedLocalBinding::new'
require_text "$tests" 'm0019_local_binding_resolution_identity_records_exact_binding'
require_text "$tests" 'm0019_local_binding_resolution_identity_distinguishes_nested_shadowing'
require_text "$tests" 'm0019_local_binding_resolution_identity_skips_unresolved_uses'

echo "m0019-local-binding-resolution-identity: identity metadata validation passed"
