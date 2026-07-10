# Task: M0021-015 Match Arm Diagnostic Provenance

## Task Metadata

- Task ID: `M0021-015`
- Milestone: `M0021`
- Milestone File: `docs/milestones/M0021-algebraic-data-and-exhaustiveness.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `Test Engineer`, then `Implementer`

## Objective

Associate duplicate qualified variant and wildcard diagnostics with the second
source match pattern, scoped to its containing `when` expression.

## Authority Extract

- `docs/SPEC.md`, “ADR-0033: Bootstrap Sealed Sums And Exhaustive Match”.
- `docs/adr/ADR-0033-bootstrap-sealed-sums-and-exhaustive-match.md`,
  “Diagnostics And Recovery”.
- `crates/newlang/src/name_resolution.rs`: `ResolvedVariantArm` and duplicate
  match-arm analysis.
- `crates/newlang/tests/name_resolution.rs`: M0021 resolution tests.

## Scope

- Preserve the resolved arm and source pattern needed for duplicate diagnostics.
- Detect duplicate variant and wildcard arms independently for each `when`.
- Diagnose the second source pattern.

## Out Of Scope

- Unknown variants, missing-variant coverage, result typing, payload patterns,
  and diagnostic rendering.

## Required Tests Before Implementation

- A second `Signal.Red` arm diagnoses on its second pattern node.
- A second wildcard diagnoses on its second wildcard pattern node.
- Equivalent arms in separate `when` expressions do not conflict.

## Acceptance Criteria

- [x] Tests fail before provenance-aware analysis exists.
- [x] Diagnostics identify the second pattern, not enum variant identity.
- [x] Duplicate detection is per `when` expression.
- [x] No coverage diagnostic is added.

## Execution Log

- 2026-07-11 agent=Main phase=task-created result=pass evidence=ADR-0033 pattern-span requirement isolated before coverage work. handoff=Test-Engineer
- 2026-07-11 agent=Main phase=test-first result=pending evidence=parser-backed duplicate-arm regression added. handoff=Implementer
- 2026-07-11 agent=Main phase=test-first result=fail evidence=provenance-aware duplicate-arm API was absent. handoff=Implementer
- 2026-07-11 agent=Main phase=implementation result=pass evidence=duplicate diagnostics traverse parsed arms per when expression. handoff=Reviewer
- 2026-07-11 agent=Main phase=ordinary-tests result=pass evidence=focused test, formatter, strict clippy, and workspace tests passed. handoff=Adversarial-Engineer
- 2026-07-11 agent=Main phase=adversarial-check result=pass evidence=parsed-arm traversal adds no safety or coverage semantics. handoff=Reviewer
- 2026-07-11 agent=Main phase=review result=approve evidence=ADR-0033 primary-pattern-span requirement and validation verified. handoff=none
