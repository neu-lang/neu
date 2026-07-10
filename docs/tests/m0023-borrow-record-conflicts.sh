#!/usr/bin/env sh
set -eu

rg -q 'pub enum BorrowKind' crates/compiler/src/borrow.rs
rg -q 'pub struct BorrowRecord' crates/compiler/src/borrow.rs
rg -q 'analyze_borrow_conflicts' crates/compiler/src/borrow.rs
rg -q 'm0023_exclusive_borrow_conflicts_with_shared_or_exclusive_in_same_region' crates/compiler/tests/borrow.rs

printf '%s\n' 'm0023 borrow record conflicts contract passed'
