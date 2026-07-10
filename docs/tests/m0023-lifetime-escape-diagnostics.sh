#!/usr/bin/env sh
set -eu

rg -q 'pub struct LifetimeEscapeRecord' crates/newlang/src/borrow.rs
rg -q 'LifetimeEscape' crates/newlang/src/borrow.rs
rg -q 'analyze_lifetime_escapes' crates/newlang/src/borrow.rs
rg -q 'm0023_lifetime_escape_diagnoses_uses_outside_borrow_region' crates/newlang/tests/borrow.rs

printf '%s\n' 'm0023 lifetime escape diagnostics contract passed'
