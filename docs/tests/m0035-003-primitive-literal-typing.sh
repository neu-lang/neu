#!/usr/bin/env bash
set -euo pipefail

grep -q 'M0035-003' docs/tasks/M0035-003-primitive-literal-typing.md
rg -q 'ByteLiteralOutOfRange' crates/compiler/src/type_check.rs
rg -q 'integer_literals' crates/compiler/src/type_check.rs
cargo test -p compiler --test type_check primitive_local_initializers_type_float_and_in_range_byte_literals
cargo test -p compiler --test type_check primitive_local_initializer_rejects_byte_literal_out_of_range
