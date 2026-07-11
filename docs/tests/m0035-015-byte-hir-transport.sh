#!/usr/bin/env bash
set -euo pipefail

grep -q 'M0035-015' docs/tasks/M0035-015-byte-hir-transport.md
rg -q 'with_byte_type|ByteLiteral' crates/compiler/src/hir.rs
cargo test -p compiler --test hir m0035_checked_source_transports_contextual_byte_literal_to_hir
