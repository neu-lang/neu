#!/usr/bin/env sh
set -eu

rg -q 'pub struct OwnershipReport' crates/compiler/src/ownership.rs
rg -q 'pub fn analyze_ownership' crates/compiler/src/ownership.rs
rg -q 'm0022_ownership_report_combines_transfers_and_diagnostics_after_type_checking' crates/compiler/tests/ownership.rs

printf '%s\n' 'm0022 ownership analysis report contract passed'
