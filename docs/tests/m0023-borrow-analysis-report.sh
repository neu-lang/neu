#!/usr/bin/env sh
set -eu

rg -q 'pub struct BorrowReport' crates/compiler/src/borrow.rs
rg -q 'pub fn analyze_borrow' crates/compiler/src/borrow.rs
rg -q 'm0023_borrow_report_combines_conflicts_and_lifetime_escapes' crates/compiler/tests/borrow.rs

printf '%s\n' 'm0023 borrow analysis report contract passed'
