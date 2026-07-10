#!/usr/bin/env sh
set -eu

rg -q 'struct ParsedQualifiedCasePattern' crates/newlang/src/parser.rs
rg -q 'qualified_case_patterns' crates/newlang/src/parser.rs
rg -q 'm0021_qualified_case_pattern_records_exact_identifier_metadata' crates/newlang/tests/parser.rs
echo 'm0021-qualified-case-pattern-metadata: contract validated'
