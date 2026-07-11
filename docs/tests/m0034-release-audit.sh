#!/usr/bin/env bash
set -euo pipefail

grep -q 'M0034-001' docs/tasks/M0034-001-release-readiness-audit.md
test -f docs/release/M0034-release-readiness.md
test -f docs/release/M0034-spec-compliance.md
test -f docs/release/M0034-diagnostics.md
test -f docs/release/M0034-build-and-target-packs.md
test -f docs/release/M0034-test-coverage.md
test -f docs/tasks/reviews/M0034-001-review.md
test -f docs/tasks/soundness/M0034-001-soundness.md
grep -q 'Known Limitations' docs/release/M0034-release-readiness.md
grep -q 'Deferred Decisions' docs/release/M0034-release-readiness.md
grep -q '\[x\] Full CI passes' docs/milestones/M0034-milestone-release-hardening.md
