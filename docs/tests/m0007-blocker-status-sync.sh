#!/usr/bin/env sh
set -eu

fail() {
  echo "m0007-status-sync: $*" >&2
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

ambiguity=docs/ambiguities/M0006-lexical-grammar.md
decision=docs/adr/proposals/reviews/ADR-0021-chief-architect-decision.md
proposal=docs/adr/proposals/ADR-0021-lexical-grammar.md
task=docs/tasks/M0007-005-lexical-grammar-blocker-status-sync.md

require_file "$ambiguity"
require_file "$decision"
require_file "$proposal"
require_file "$task"
require_file docs/adr/ADR-0021-lexical-grammar.md
require_file docs/adr/proposals/reviews/ADR-0021-language-designer-review.md
require_file docs/adr/proposals/reviews/ADR-0021-adversarial-review.md
require_file docs/adr/proposals/reviews/ADR-0021-diagnostics-review.md
require_file docs/adr/proposals/reviews/ADR-0021-simplicity-review.md

require_text "$proposal" '^Status: Draft proposal - not accepted source of truth$'
require_text docs/adr/ADR-0021-lexical-grammar.md '^Status: Accepted$'
require_text "$ambiguity" 'Status: `resolved`'
require_text "$ambiguity" 'main-task semantic design drafted non-authoritative ADR proposal and ownership review'
require_text "$ambiguity" 'main-task adversarial check reviewed soundness risk'
require_text "$ambiguity" 'main-task diagnostics check reviewed diagnostic consequences'
require_text "$ambiguity" 'main-task simplicity check reviewed complexity'
require_text "$ambiguity" '\[x\] main task approves final resolution'
require_text "$ambiguity" 'Accepted `docs/adr/ADR-0021-lexical-grammar.md`'

require_text "$decision" '^Decision: approved$'
require_text "$decision" '^## Completed Review Dependencies$'
require_text "$decision" 'main-task semantic design ownership review'
require_text "$decision" 'main-task adversarial check soundness review'
require_text "$decision" 'main-task diagnostics check review'
require_text "$decision" 'main-task simplicity check review'
require_text "$decision" '^## Resolved Acceptance Blockers$'
require_text "$decision" 'Concrete accepted lexical grammar'
require_text "$decision" 'Accepted `docs/adr/ADR-0021-lexical-grammar.md` as source of truth'

require_absent_path crates/compiler/src/token.rs

echo "m0007-status-sync: lexical grammar blocker resolved-state validation passed"
