#!/usr/bin/env bash
set -euo pipefail

grep -q 'M0035-013' docs/tasks/M0035-013-primitive-type-integration.md
rg -q 'type_m0035_primitive_operators' crates/compiler/src/type_check.rs
cargo test -p compiler --test type_check m0035_executable_core_types_primitive_operator_source
cargo test -p compiler --test type_check m0035_executable_core_rejects_invalid_primitive_operator_source
