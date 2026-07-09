#!/usr/bin/env sh
set -eu

fail() {
  echo "m0011-accepted: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_path() {
  [ ! -e "$1" ] || fail "out-of-scope parser artifact exists during ADR acceptance: $1"
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

require_not_text() {
  file="$1"
  pattern="$2"
  if grep -Eq "$pattern" "$file"; then
    fail "unexpected pattern in $file: $pattern"
  fi
}

adr=docs/adr/ADR-0022-declaration-syntax.md
spec=docs/SPEC.md
ambiguity=docs/ambiguities/M0008-declaration-syntax.md
decision=docs/adr/proposals/reviews/ADR-0022-chief-architect-decision.md
ledger=docs/syntax/grammar-authority-ledger.md
task=docs/tasks/M0011-005-accept-declaration-syntax-adr.md

require_file "$adr"
require_file "$spec"
require_file "$ambiguity"
require_file "$decision"
require_file "$ledger"
require_file "$task"

require_text "$adr" '^# ADR-0022: Declaration Syntax$'
require_text "$adr" '^Status: Accepted$'
require_text "$adr" '^## Concrete Grammar$'
require_text "$adr" '^### Source File Order$'
require_text "$adr" '^### Package Declarations$'
require_text "$adr" '^### Import Declarations$'
require_text "$adr" '^### Visibility And Modifiers$'
require_text "$adr" '^### Function Declarations$'
require_text "$adr" '^### Struct Declarations$'
require_text "$adr" '^### Enum Or Sealed Sum Declarations$'
require_text "$adr" '^### Interface Declarations$'
require_text "$adr" '^### Declaration Diagnostics$'
require_text "$adr" '^### Explicit Deferrals$'
require_text "$adr" 'source-file = package-declaration\? import-declaration\* top-level-declaration\*'
require_text "$adr" 'package-declaration = `package` qualified-name'
require_text "$adr" 'import-declaration = `import` qualified-name import-alias\?'
require_text "$adr" 'visibility = `public` \| `private` \| `internal`'
require_text "$adr" 'function-declaration = visibility\? `fun` identifier parameter-list return-type-placeholder\? function-body-placeholder'
require_text "$adr" 'struct-declaration = visibility\? `struct` identifier declaration-body'
require_text "$adr" 'enum-declaration = visibility\? `enum` identifier declaration-body'
require_text "$adr" 'interface-declaration = visibility\? `interface` identifier declaration-body'
require_text "$adr" 'misplaced_package_declaration'
require_text "$adr" 'duplicate_visibility_modifier'
require_text "$adr" 'unsupported_declaration_modifier'
require_text "$adr" 'skip-to-declaration-boundary'
require_text "$adr" 'All declaration diagnostics must cite ADR-0015 and ADR-0022'

require_text "$spec" '^## ADR-0022: Declaration Syntax$'
require_text "$spec" 'package declarations'
require_text "$spec" 'import declarations'
require_text "$spec" 'visibility modifiers'
require_text "$spec" 'structs, enums or sealed sums, interfaces, and declaration bodies'

require_text "$ambiguity" 'Status: `resolved`'
require_text "$ambiguity" 'Source of truth updated:'
require_text "$ambiguity" 'docs/adr/ADR-0022-declaration-syntax.md'
require_text "$ambiguity" 'Date resolved:'
require_text "$ambiguity" '2026-07-09'
require_not_text "$ambiguity" 'unresolved'

require_text "$decision" '^Decision: approved$'
require_text "$decision" 'Accepted source of truth: `docs/adr/ADR-0022-declaration-syntax.md`'
require_text "$decision" 'M0011 declaration parser fixture and implementation tasks may proceed'

require_text "$ledger" '\| Package declaration \| specified \| ADR-0022'
require_text "$ledger" '\| Import declaration \| specified \| ADR-0022'
require_text "$ledger" '\| Visibility modifier syntax \| specified \| ADR-0022'
require_text "$ledger" '\| Function declaration \| specified \| ADR-0022'
require_text "$ledger" '\| Struct declaration \| specified \| ADR-0022'
require_text "$ledger" '\| Enum or sealed sum declaration \| specified \| ADR-0022'
require_text "$ledger" '\| Interface declaration \| specified \| ADR-0022'
require_text "$ledger" '\| Type declaration \| ambiguous \| ADR-0010'
require_text "$ledger" '\| Expression grammar \| ambiguous \| none'
require_text "$ledger" '\| Statement grammar \| ambiguous \| none'
require_text "$ledger" '\| Pattern grammar \| ambiguous \| ADR-0012'
require_text "$ledger" 'M0011 declaration parser may proceed only for ADR-0022 constructs'

require_text "$task" 'Status: `complete`'
require_text "$task" 'Tests fail before implementation for the expected reason'
require_text "$task" 'CI passes as final gate'


echo "m0011-accepted: declaration syntax accepted and parser implementation still deferred"
