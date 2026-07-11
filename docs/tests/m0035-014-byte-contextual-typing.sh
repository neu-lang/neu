#!/usr/bin/env bash
set -euo pipefail

grep -q 'M0035-014' docs/tasks/M0035-014-byte-contextual-typing.md
rg -q 'ByteLiteralOutOfRange|integer_literals' crates/compiler/src/type_check.rs
cargo test -p compiler --test type_check m0035_executable_core_types_byte_literal_in_context
cargo test -p compiler --test type_check m0035_executable_core_rejects_byte_literal_out_of_range
