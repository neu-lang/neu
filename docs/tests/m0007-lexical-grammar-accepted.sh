#!/usr/bin/env sh
set -eu

fail() {
  echo "m0007-accepted-grammar: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope path exists before lexer implementation task: $1"
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

adr=docs/adr/ADR-0021-lexical-grammar.md
spec=docs/SPEC.md
ambiguity=docs/ambiguities/M0006-lexical-grammar.md
decision=docs/adr/proposals/reviews/ADR-0021-chief-architect-decision.md
task=docs/tasks/M0007-006-accept-lexical-grammar-adr.md

require_file "$adr"
require_file "$spec"
require_file "$ambiguity"
require_file "$decision"
require_file "$task"

require_text "$adr" '^# ADR-0021: Lexical Grammar$'
require_text "$adr" '^Status: Accepted$'
require_text "$adr" '^## Accepted Lexical Grammar$'
require_text "$adr" '^### Source Text$'
require_text "$adr" '^### Whitespace$'
require_text "$adr" '^### Comments$'
require_text "$adr" '^### Identifiers$'
require_text "$adr" '^### Keywords$'
require_text "$adr" '^### Integer Literals$'
require_text "$adr" '^### String Literals$'
require_text "$adr" '^### Operators And Delimiters$'
require_text "$adr" '^### Lexical Error Categories$'
require_text "$adr" '^### Source Span Rules$'
require_text "$adr" 'small Kotlin-like custom lexical grammar'
require_text "$adr" 'Unicode identifiers are deferred'
require_text "$adr" 'Block comments nest'
require_text "$adr" 'Integer literal overflow is not a lexer error'
require_text "$adr" 'ADR-0015'

require_text "$spec" '^## ADR-0021: Lexical Grammar$'
require_text "$spec" 'accepted small Kotlin-like custom lexical grammar'

require_text "$ambiguity" 'Status: `resolved`'
require_text "$ambiguity" 'Decision:[[:space:]]*$'
require_text "$ambiguity" 'Accepted `docs/adr/ADR-0021-lexical-grammar.md`'
require_text "$ambiguity" 'Source of truth updated:[[:space:]]*$'
require_text "$ambiguity" '`docs/adr/ADR-0021-lexical-grammar.md`'
require_text "$ambiguity" 'Date resolved:[[:space:]]*$'
require_text "$ambiguity" '2026-07-09'

require_text "$decision" '^Decision: approved$'
require_text "$decision" 'Accepted `docs/adr/ADR-0021-lexical-grammar.md` as source of truth'
require_text "$decision" 'M0007 may proceed to concrete lexer fixtures and implementation tasks'

require_absent_path crates/newlang/src/lexer.rs
require_absent_path crates/newlang/src/token.rs
require_absent_path tests/fixtures/lexer/keywords.fixture.toml
require_absent_path tests/fixtures/lexer/identifiers.fixture.toml
require_absent_path tests/fixtures/lexer/literals.fixture.toml

echo "m0007-accepted-grammar: accepted lexical grammar validation passed"
