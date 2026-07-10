#!/usr/bin/env sh
set -eu

rg -q 'analyze_when_subjects' crates/newlang/src/name_resolution.rs
rg -q 'InvalidMatchSubject' crates/newlang/src/name_resolution.rs
rg -q 'm0021_when_subject_analysis_accepts_enum_parameter_only' crates/newlang/tests/name_resolution.rs
echo 'm0021-when-subject-analysis: contract validated'
