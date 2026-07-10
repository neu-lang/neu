#!/usr/bin/env sh
set -eu

rg -q 'pub enum OwnershipDiagnosticKind' crates/compiler/src/ownership.rs
rg -q 'UseAfterMove' crates/compiler/src/ownership.rs
rg -q 'analyze_use_after_move' crates/compiler/src/ownership.rs
rg -q 'm0022_use_after_move_diagnostics_report_later_uses_and_origin' crates/compiler/tests/ownership.rs

printf '%s\n' 'm0022 use-after-move diagnostics contract passed'
