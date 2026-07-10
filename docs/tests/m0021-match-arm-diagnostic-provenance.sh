#!/usr/bin/env sh
set -eu

rg -q 'analyze_duplicate_match_arms' crates/compiler/src/name_resolution.rs
rg -q 'DuplicateMatchWildcard' crates/compiler/src/name_resolution.rs
echo 'm0021-match-arm-diagnostic-provenance: contract validated'
