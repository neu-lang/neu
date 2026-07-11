#!/usr/bin/env bash
set -euo pipefail

grep -q 'M0035-016' docs/tasks/M0035-016-primitive-function-signatures.md
rg -q '"Float" => primitives.float_id|"Byte" => primitives.byte_id' crates/compiler/src/type_check.rs
cargo test -p compiler --test type_check m0035_function_signatures_type_all_bootstrap_primitives
