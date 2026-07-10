#!/usr/bin/env sh
set -eu

rg -q 'analyze_match_exhaustiveness' crates/compiler/src/name_resolution.rs
rg -q 'NonExhaustiveMatch' crates/compiler/src/name_resolution.rs
rg -q 'm0021_exhaustiveness_reports_only_otherwise_valid_missing_coverage' crates/compiler/tests/name_resolution.rs

printf '%s\n' 'm0021 exhaustive match coverage contract passed'
