#!/usr/bin/env sh
set -eu

fail() {
  echo "roadmap-executable-semantics-ordering: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_absent_file() {
  [ ! -f "$1" ] || fail "obsolete milestone file still exists: $1"
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

require_file docs/milestones/M0027-executable-semantics-planning.md
require_file docs/milestones/M0028-executable-expression-frontend-completion.md
require_file docs/milestones/M0029-hir-design-and-lowering.md
require_file docs/milestones/M0030-mir-design-and-lowering.md
require_file docs/milestones/M0031-cranelift-backend-smoke.md
require_file docs/milestones/M0032-object-and-bundled-linker-pipeline.md
require_file docs/milestones/M0033-target-packs-and-cross-compilation-smoke.md
require_file docs/milestones/M0034-milestone-release-hardening.md

require_absent_file docs/milestones/M0027-hir-design-and-lowering.md
require_absent_file docs/milestones/M0028-mir-design-and-lowering.md
require_absent_file docs/milestones/M0029-cranelift-backend-smoke.md
require_absent_file docs/milestones/M0030-object-and-bundled-linker-pipeline.md
require_absent_file docs/milestones/M0031-target-packs-and-cross-compilation-smoke.md
require_absent_file docs/milestones/M0032-milestone-release-hardening.md

require_text docs/ROADMAP.md 'M0027: Executable Semantics Planning'
require_text docs/ROADMAP.md 'M0028: Executable Expression Frontend Completion'
require_text docs/ROADMAP.md 'M0029: HIR Design And Lowering'
require_text docs/ROADMAP.md 'M0034: Milestone Release Hardening'
require_text docs/ROADMAP.md 'M0034 because every phase builds'

require_text docs/milestones/M0028-executable-expression-frontend-completion.md 'No HIR, MIR, backend, object, or linker implementation is added'
require_text docs/milestones/M0031-cranelift-backend-smoke.md 'ADR-0043 arithmetic, exponentiation, bitwise, and'
require_text docs/milestones/M0032-object-and-bundled-linker-pipeline.md 'ADR-0040 `main` result as the process exit code'
require_text docs/milestones/M0034-milestone-release-hardening.md 'All completed milestones M0001-M0033'

echo "roadmap executable semantics ordering validation passed"
