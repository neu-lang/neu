#!/usr/bin/env sh
set -eu

rg -q 'pub enum BorrowKind' crates/newlang/src/borrow.rs
rg -q 'pub struct BorrowRecord' crates/newlang/src/borrow.rs
rg -q 'analyze_borrow_conflicts' crates/newlang/src/borrow.rs
rg -q 'm0023_exclusive_borrow_conflicts_with_shared_or_exclusive_in_same_region' crates/newlang/tests/borrow.rs

printf '%s\n' 'm0023 borrow record conflicts contract passed'
