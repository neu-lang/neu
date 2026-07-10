#!/usr/bin/env sh
set -eu

rg -q 'struct EnumVariantIdentity' crates/compiler/src/name_resolution.rs
rg -q 'struct EnumVariantIndex' crates/compiler/src/name_resolution.rs
rg -q 'fn build_enum_variant_index' crates/compiler/src/name_resolution.rs
rg -q 'm0021_enum_variant_identity_preserves_enum_and_variant_source_order' crates/compiler/tests/name_resolution.rs

echo 'm0021-enum-variant-identity: contract validated'
