#!/usr/bin/env bash
set -euo pipefail

grep -q 'M0033-004' docs/tasks/M0033-004-target-pack-inventory.md
rg -q 'available_targets' crates/compiler/src/target_pack.rs
cargo test -p compiler --test target_pack_inventory
