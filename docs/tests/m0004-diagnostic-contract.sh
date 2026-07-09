#!/usr/bin/env sh
set -eu

fail() {
  echo "m0004: $*" >&2
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

require_absent_text() {
  file="$1"
  pattern="$2"
  if grep -Eq "$pattern" "$file"; then
    fail "unexpected pattern in $file: $pattern"
  fi
}

require_file docs/diagnostics.md
require_file tests/golden/diagnostics/M0004-inert.diagnostic.toml

require_text docs/diagnostics.md 'ADR-0015'
require_text docs/diagnostics.md 'primary span'
require_text docs/diagnostics.md 'secondary span'
require_text docs/diagnostics.md 'severity'
require_text docs/diagnostics.md 'notes'
require_text docs/diagnostics.md 'safe suggestion'
require_text docs/diagnostics.md 'internal compiler jargon'
require_text docs/diagnostics.md 'docs/SPEC\.md'
require_text docs/diagnostics.md 'docs/adr/'

require_text tests/golden/diagnostics/M0004-inert.diagnostic.toml '^kind = "inert-diagnostic-shape"$'
require_text tests/golden/diagnostics/M0004-inert.diagnostic.toml '^milestone = "M0004"$'
require_text tests/golden/diagnostics/M0004-inert.diagnostic.toml '^compiler_behavior = "none"$'
require_text tests/golden/diagnostics/M0004-inert.diagnostic.toml '^severity = "error"$'
require_text tests/golden/diagnostics/M0004-inert.diagnostic.toml '^primary_span = "required"$'
require_text tests/golden/diagnostics/M0004-inert.diagnostic.toml '^secondary_spans = "optional"$'
require_text tests/golden/diagnostics/M0004-inert.diagnostic.toml '^notes = "optional"$'
require_text tests/golden/diagnostics/M0004-inert.diagnostic.toml '^safe_suggestions = "optional"$'
require_text tests/golden/diagnostics/M0004-inert.diagnostic.toml '^source = "docs/adr/ADR-0015-diagnostics-as-semantics.md"$'

require_absent_text tests/golden/diagnostics/M0004-inert.diagnostic.toml 'lexer|parser|type checker|ownership|borrow|lifetime|thread|coroutine|FFI|unsafe'

echo "m0004: diagnostic contract validation passed"
