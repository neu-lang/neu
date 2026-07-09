#!/usr/bin/env sh
set -eu

fail() {
  echo "m0010-parser-recovery: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope path exists during M0010 parser recovery architecture: $1"
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

recovery=docs/parser/recovery.md
fixtures=docs/parser/syntax-diagnostic-fixtures.md
fixture=tests/fixtures/diagnostics/M0010-synthetic-parser-error.fixture.toml
golden=tests/golden/diagnostics/M0010-synthetic-parser-error.diagnostic.toml
milestone=docs/milestones/M0010-parser-recovery-architecture.md

require_file "$recovery"
require_file "$fixtures"
require_file "$fixture"
require_file "$golden"
require_file "$milestone"
require_file docs/diagnostics.md
require_file docs/syntax/grammar-authority-ledger.md

require_text "$recovery" '^# Parser Recovery Architecture$'
require_text "$recovery" 'Status: M0010 architecture'
require_text "$recovery" 'No ambiguous syntax is parsed'
require_text "$recovery" 'synchronization token'
require_text "$recovery" 'primary span'
require_text "$recovery" 'ADR-0015'
require_text "$recovery" 'ADR-0021'
require_text "$recovery" 'docs/syntax/grammar-authority-ledger.md'
require_text "$recovery" 'synthetic parser error'

require_text "$fixtures" '^# Syntax Diagnostic Fixture Format$'
require_text "$fixtures" 'kind = "synthetic-parser-diagnostic-fixture"'
require_text "$fixtures" 'compiler_behavior = "synthetic-parser-diagnostic-only"'
require_text "$fixtures" 'must not encode declaration, type, expression, statement, or pattern grammar'

require_text "$fixture" '^kind = "synthetic-parser-diagnostic-fixture"$'
require_text "$fixture" '^milestone = "M0010"$'
require_text "$fixture" '^compiler_behavior = "synthetic-parser-diagnostic-only"$'
require_text "$fixture" 'source = "docs/milestones/M0010-parser-recovery-architecture.md"'
require_text "$fixture" 'synthetic_unexpected_token'
require_text "$fixture" 'primary_span = "3..4"'
require_text "$fixture" 'recovery_action = "skip-to-synchronization-token"'
require_absent_text "$fixture" 'fun|val|var|if|when|struct|enum|interface'

require_text "$golden" '^severity = "error"$'
require_text "$golden" 'message = "unexpected token in synthetic parser input"'
require_text "$golden" 'primary_span = "3..4"'
require_text "$golden" 'source_of_truth = "docs/milestones/M0010-parser-recovery-architecture.md"'
require_text "$golden" 'recovery_action = "skip-to-synchronization-token"'

require_text "$milestone" '\[x\] Recovery strategy is documented'
require_text "$milestone" '\[x\] Syntax diagnostic shape is tested'
require_text "$milestone" '\[x\] Ambiguous syntax remains blocked'

require_absent_path crates/newlang/src/parser.rs
require_absent_path tests/fixtures/parser

echo "m0010-parser-recovery: parser recovery architecture validation passed"
