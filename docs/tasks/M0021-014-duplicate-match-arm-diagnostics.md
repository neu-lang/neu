# Task: M0021-014 Duplicate Match Arm Diagnostics

## Task Metadata

- Task ID: `M0021-014`
- Milestone: `M0021`
- Milestone File: `docs/milestones/M0021-algebraic-data-and-exhaustiveness.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `Test Engineer`, then `Implementer`

## Objective

Diagnose repeated resolved variant arms as `duplicate_match_variant` and a
second wildcard arm as `duplicate_match_wildcard`.

## Authority Extract

- `docs/SPEC.md`, “ADR-0033: Bootstrap Sealed Sums And Exhaustive Match”.
- `docs/adr/ADR-0033-bootstrap-sealed-sums-and-exhaustive-match.md`,
  “Diagnostics And Recovery”.
- `crates/compiler/src/name_resolution.rs`: resolved arms and match report.

## Scope

- Per-`when` duplicate resolved variants and wildcard arms.

## Out Of Scope

- Unknown arms, non-exhaustiveness, result typing, and payload patterns.

## Required Tests Before Implementation

- A second resolved variant arm diagnoses on its pattern.
- A second wildcard arm diagnoses on its wildcard pattern.

## Acceptance Criteria

- [x] Tests fail before duplicate-arm analysis exists.
- [x] First valid arm remains canonical.
- [x] No missing-variant diagnostic is added.

## Execution Log

- 2026-07-11 agent=Main phase=test-first result=fail evidence=duplicate arm analyzer was absent. handoff=Implementer
- 2026-07-11 agent=Main phase=implementation result=pass evidence=second variant and wildcard arms diagnose. handoff=Reviewer
- 2026-07-11 agent=Main phase=ordinary-tests result=pass evidence=focused test, formatter, strict clippy, and workspace tests passed. handoff=Adversarial-Engineer
- 2026-07-11 agent=Main phase=adversarial-check result=pass evidence=first arms remain canonical; no coverage semantics are introduced. handoff=Reviewer
- 2026-07-11 agent=Main phase=review result=approve evidence=ADR-0033 duplicate-arm scope and validation verified. handoff=none
