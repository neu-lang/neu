#!/usr/bin/env bash
set -euo pipefail

grep -q 'ADR-0059' docs/SPEC.md
grep -q 'M0035' docs/ROADMAP.md
test -f docs/adr/ADR-0059-bootstrap-primitive-runtime-support.md
test -f docs/milestones/M0035-primitive-runtime-support.md
grep -q 'Status: Accepted' docs/adr/ADR-0059-bootstrap-primitive-runtime-support.md
grep -q 'M0035' docs/adr/proposals/reviews/ADR-0059-chief-architect-decision.md
