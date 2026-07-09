#!/usr/bin/env sh
set -eu

fail() {
  echo "m0011-concrete-draft-history: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope parser artifact exists during draft history validation: $1"
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

proposal=docs/adr/proposals/ADR-0022-declaration-syntax.md
ambiguity=docs/ambiguities/M0008-declaration-syntax.md
decision=docs/adr/proposals/reviews/ADR-0022-chief-architect-decision.md

require_file "$proposal"
require_file "$ambiguity"
require_file "$decision"

require_text "$proposal" '^Status: Draft proposal - not accepted source of truth$'
require_text "$proposal" '^## Concrete Draft Grammar$'
require_text "$proposal" '^### Source File Order$'
require_text "$proposal" '^### Qualified Names$'
require_text "$proposal" '^### Package Declarations$'
require_text "$proposal" '^### Import Declarations$'
require_text "$proposal" '^### Visibility And Modifiers$'
require_text "$proposal" '^### Function Declarations$'
require_text "$proposal" '^### Struct Declarations$'
require_text "$proposal" '^### Enum Or Sealed Sum Declarations$'
require_text "$proposal" '^### Interface Declarations$'
require_text "$proposal" '^### Declaration Terminators And Recovery$'
require_text "$proposal" '^### Declaration Diagnostics$'
require_text "$proposal" '^### Explicit Deferrals$'
require_text "$proposal" 'source-file = package-declaration\? import-declaration\* top-level-declaration\*'
require_text "$proposal" 'package-declaration = `package` qualified-name'
require_text "$proposal" 'import-declaration = `import` qualified-name import-alias\?'
require_text "$proposal" 'visibility = `public` \| `private` \| `internal`'
require_text "$proposal" 'function-declaration = visibility\? `fun` identifier parameter-list return-type-placeholder\? function-body-placeholder'
require_text "$proposal" 'struct-declaration = visibility\? `struct` identifier declaration-body'
require_text "$proposal" 'interface-declaration = visibility\? `interface` identifier declaration-body'
require_text "$proposal" 'misplaced_package_declaration'
require_text "$proposal" 'missing_declaration_name'
require_text "$proposal" 'primary span'
require_text "$proposal" 'skip-to-declaration-boundary'
require_text "$proposal" 'not accepted source of truth'

require_text "$ambiguity" 'Status: `resolved`'
require_text "$ambiguity" 'docs/adr/ADR-0022-declaration-syntax.md'
require_text "$decision" '^Decision: approved$'
require_text "$decision" 'Accepted source of truth: `docs/adr/ADR-0022-declaration-syntax.md`'

require_absent_path crates/newlang/src/parser.rs
require_absent_path tests/fixtures/parser

echo "m0011-concrete-draft-history: declaration syntax concrete draft history validation passed"
