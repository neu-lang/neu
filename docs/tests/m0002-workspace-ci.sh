#!/usr/bin/env sh
set -eu

fail() {
  echo "m0002: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope path exists for M0002: $1"
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

require_file Cargo.toml
require_file rust-toolchain.toml
require_file crates/newlang/Cargo.toml
require_file crates/newlang/src/lib.rs
require_file .github/workflows/ci.yml
require_file docs/build.md

require_text Cargo.toml '^\[workspace\]$'
require_text Cargo.toml '^members = \["crates/newlang"\]$'
require_text crates/newlang/Cargo.toml '^name = "newlang"$'
require_text docs/build.md 'cargo fmt --all --check'
require_text docs/build.md 'cargo clippy --workspace --all-targets -- -D warnings'
require_text docs/build.md 'cargo test --workspace --all-targets'
require_text .github/workflows/ci.yml 'cargo fmt --all --check'
require_text .github/workflows/ci.yml 'cargo clippy --workspace --all-targets -- -D warnings'
require_text .github/workflows/ci.yml 'cargo test --workspace --all-targets'

require_absent_path crates/newlang/src/parser.rs
require_absent_path crates/newlang/src/ast.rs
require_absent_path crates/newlang/src/hir.rs
require_absent_path crates/newlang/src/mir.rs
require_absent_path crates/newlang/src/backend

cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets

echo "m0002: workspace and CI skeleton validation passed"
