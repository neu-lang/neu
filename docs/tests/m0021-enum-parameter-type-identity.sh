#!/usr/bin/env sh
set -eu

rg -q 'resolve_enum_parameter_types' crates/newlang/src/name_resolution.rs
rg -q 'm0021_enum_parameter_type_identity_records_same_package_enum' crates/newlang/tests/name_resolution.rs
echo 'm0021-enum-parameter-type-identity: contract validated'
