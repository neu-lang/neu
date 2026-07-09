#!/usr/bin/env sh
set -eu

fail() {
  echo "m0005: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope path exists for M0005: $1"
}

require_file crates/newlang/src/source.rs
require_file crates/newlang/src/lib.rs
require_file docs/tasks/M0005-001-source-spans.md

require_text crates/newlang/src/lib.rs '^pub mod source;$'
require_text crates/newlang/src/source.rs 'SourceDatabase'
require_text crates/newlang/src/source.rs 'SourceFileId'
require_text crates/newlang/src/source.rs 'ByteSpan'
require_text crates/newlang/src/source.rs 'LineColumn'
require_text docs/tasks/M0005-001-source-spans.md 'Unicode column semantics are unresolved'

require_absent_path crates/newlang/src/parser.rs
require_absent_path crates/newlang/src/hir.rs
require_absent_path crates/newlang/src/mir.rs
require_absent_path crates/newlang/src/backend

cargo test --workspace --all-targets source::tests

echo "m0005: source span validation passed"
