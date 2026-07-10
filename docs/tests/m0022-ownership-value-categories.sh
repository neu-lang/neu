#!/usr/bin/env sh
set -eu

rg -q 'pub enum OwnershipCategory' crates/compiler/src/ownership.rs
rg -q 'classify_ownership_category' crates/compiler/src/ownership.rs
rg -q 'm0022_primitive_ownership_categories_follow_adr0035' crates/compiler/tests/ownership.rs

printf '%s\n' 'm0022 ownership value categories contract passed'
