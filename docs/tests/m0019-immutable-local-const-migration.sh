#!/usr/bin/env sh
set -eu

fail() {
  echo "m0019-immutable-local-const-migration: $*" >&2
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
    fail "forbidden stale or spelling-specific metadata in $file: $pattern"
  fi
}

for file in \
  crates/compiler/tests/lexer.rs \
  crates/compiler/tests/parser.rs \
  crates/compiler/tests/name_resolution.rs \
  crates/compiler/tests/type_check.rs \
  tests/fixtures/lexer/keywords.fixture.toml \
  tests/fixtures/parser/statements/positive.fixture.toml \
  tests/fixtures/parser/statements/negative.fixture.toml \
  examples/current/type_checked.neu \
  examples/current/accepted_nullability_flow.neu \
  examples/current/parsed_surface.neu
do
  require_file "$file"
done

require_text crates/compiler/src/lexer.rs 'KwConst'
require_text crates/compiler/src/parser.rs 'TokenKind::KwConst'
require_text crates/compiler/src/name_resolution.rs '^    Immutable,$'
require_text crates/compiler/src/type_check.rs 'LocalBindingKind::Immutable'

require_text crates/compiler/tests/lexer.rs 'TokenKind::KwConst, TokenKind::Identifier, TokenKind::KwVar'
require_text crates/compiler/tests/parser.rs 'const val: Int = 1; var val = 2;'
require_text crates/compiler/tests/parser.rs 'removed_val_introducer_uses_ordinary_recovery_without_a_binding_alias'
require_text crates/compiler/tests/parser.rs 'DiagnosticKind::UnexpectedTokenInStatement'
require_text crates/compiler/tests/name_resolution.rs 'LocalBindingKind::Immutable'
require_text crates/compiler/tests/type_check.rs 'const maybe: String\? = null'
require_text tests/fixtures/lexer/keywords.fixture.toml 'expected_tokens = \["IDENTIFIER"\]'
require_text tests/fixtures/parser/statements/negative.fixture.toml 'removed_val_introducer_uses_ordinary_statement_recovery'
require_text examples/current/type_checked.neu '^    const ready: Bool = true;$'
require_text examples/current/accepted_nullability_flow.neu '^    const maybe: String\? = null;$'
require_text examples/current/accepted_nullability_flow.neu '^        const definite: String = maybe;$'
require_text examples/current/parsed_surface.neu '^    const outer = one\(\);$'
require_text examples/current/parsed_surface.neu '^        const inner = outer;$'
require_text examples/current/parsed_surface.neu '^    public val size: Int$'

require_absent_text crates/compiler/src/lexer.rs 'KwVal'
require_absent_text crates/compiler/src/parser.rs 'KwVal|LocalBindingKind::Val'
require_absent_text crates/compiler/src/name_resolution.rs 'LocalBindingKind::Val|LegacyVal|ValBinding'
require_absent_text crates/compiler/src/type_check.rs 'LocalBindingKind::Val|ConstBinding|ConstValue|ConstantValue|ConstEvaluator|StaticStorage|LegacyVal'
require_absent_text crates/compiler/tests/name_resolution.rs 'LocalBindingKind::Val'
require_absent_text crates/compiler/tests/type_check.rs 'LocalBindingKind::Val'
require_absent_text crates/compiler/src/lexer.rs 'LegacyVal|Legacy.*Diagnostic'
require_absent_text crates/compiler/src/parser.rs 'LegacyVal|Legacy.*Diagnostic'
require_absent_text examples/current/type_checked.neu '^[[:space:]]+val[[:space:]]'
require_absent_text examples/current/accepted_nullability_flow.neu '^[[:space:]]+val[[:space:]]'
require_absent_text examples/current/parsed_surface.neu '^[[:space:]]+val[[:space:]]'

echo "m0019-immutable-local-const-migration: cross-phase contract validated"
