#!/usr/bin/env sh
set -eu

rg -q 'pub enum OwnershipCategory' crates/newlang/src/ownership.rs
rg -q 'classify_ownership_category' crates/newlang/src/ownership.rs
rg -q 'm0022_primitive_ownership_categories_follow_adr0035' crates/newlang/tests/ownership.rs

printf '%s\n' 'm0022 ownership value categories contract passed'
