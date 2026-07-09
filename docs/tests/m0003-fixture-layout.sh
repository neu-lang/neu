#!/usr/bin/env sh
set -eu

fail() {
  echo "m0003: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_dir() {
  [ -d "$1" ] || fail "missing required directory: $1"
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope path exists for M0003: $1"
}

require_dir tests/fixtures/positive
require_dir tests/fixtures/negative
require_dir tests/fixtures/diagnostics
require_dir tests/golden/diagnostics

require_file tests/fixtures/positive/M0003-inert.fixture.toml
require_file tests/fixtures/negative/.gitkeep
require_file tests/fixtures/diagnostics/.gitkeep
require_file tests/golden/diagnostics/.gitkeep
require_file docs/test-harness.md

require_text tests/fixtures/positive/M0003-inert.fixture.toml '^kind = "inert"$'
require_text tests/fixtures/positive/M0003-inert.fixture.toml '^milestone = "M0003"$'
require_text tests/fixtures/positive/M0003-inert.fixture.toml '^source = "docs/milestones/M0003-test-harness-and-golden-fixture-layout.md"$'
require_text tests/fixtures/positive/M0003-inert.fixture.toml '^compiler_behavior = "none"$'

require_text docs/test-harness.md 'docs/SPEC\.md'
require_text docs/test-harness.md 'docs/adr/'
require_text docs/test-harness.md 'positive'
require_text docs/test-harness.md 'negative'
require_text docs/test-harness.md 'diagnostics'
require_text docs/test-harness.md 'golden'
require_text docs/test-harness.md 'compiler behavior'

if grep -Eq 'source_text|expected_output|expected_error|token|parse|lexer|parser|AST|HIR|MIR' tests/fixtures/positive/M0003-inert.fixture.toml; then
  fail "inert fixture must not encode compiler behavior, source syntax, or expected compiler output"
fi

require_absent_path crates/newlang/src/hir.rs
require_absent_path crates/newlang/src/mir.rs
require_absent_path crates/newlang/src/backend

echo "m0003: fixture layout validation passed"
