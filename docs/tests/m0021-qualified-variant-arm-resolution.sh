#!/usr/bin/env sh
set -eu
rg -q 'resolve_qualified_variant_arms' crates/compiler/src/name_resolution.rs
rg -q 'UnknownMatchVariant' crates/compiler/src/name_resolution.rs
echo 'm0021-qualified-variant-arm-resolution: contract validated'
