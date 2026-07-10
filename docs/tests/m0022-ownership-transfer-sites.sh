#!/usr/bin/env sh
set -eu

rg -q 'pub enum OwnershipTransferKind' crates/compiler/src/ownership.rs
rg -q 'collect_ownership_transfers' crates/compiler/src/ownership.rs
rg -q 'm0022_transfer_sites_record_only_move_only_local_sources' crates/compiler/tests/ownership.rs

printf '%s\n' 'm0022 ownership transfer sites contract passed'
