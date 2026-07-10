#!/usr/bin/env sh
set -eu

rg -q 'pub struct ThreadBoundary' crates/compiler/src/thread.rs
rg -q 'pub struct ThreadCapture' crates/compiler/src/thread.rs
rg -q 'pub enum ThreadDiagnosticKind' crates/compiler/src/thread.rs
rg -q 'MissingThreadCapability' crates/compiler/src/thread.rs
rg -q 'analyze_thread_boundaries' crates/compiler/src/thread.rs
rg -q 'm0024_boundary_analysis_reports_missing_capabilities' crates/compiler/tests/thread.rs
rg -q 'm0024_boundary_diagnostics_preserve_order_and_spans' crates/compiler/tests/thread.rs

cargo test -p compiler --test thread m0024_boundary

printf '%s\n' 'm0024 thread boundary capture diagnostics contract passed'
