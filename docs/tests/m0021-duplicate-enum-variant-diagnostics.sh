#!/usr/bin/env sh
set -eu
rg -q 'analyze_duplicate_enum_variants' crates/compiler/src/name_resolution.rs
rg -q 'DuplicateEnumVariant' crates/compiler/src/name_resolution.rs
echo 'm0021-duplicate-enum-variant-diagnostics: contract validated'
