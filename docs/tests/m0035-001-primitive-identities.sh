#!/usr/bin/env bash
set -euo pipefail

grep -q 'M0035-001' docs/tasks/M0035-001-primitive-type-identities.md
rg -q 'Float' crates/compiler/src/types.rs crates/compiler/src/type_check.rs
rg -q 'Byte' crates/compiler/src/types.rs crates/compiler/src/type_check.rs
cargo test -p compiler --test type_check primitive_local_declaration_annotations_record_declaration_signatures
