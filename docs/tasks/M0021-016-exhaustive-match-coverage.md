# Task: M0021-016 Exhaustive Match Coverage

## Task Metadata

- Task ID: `M0021-016`
- Milestone: `M0021`
- Milestone File: `docs/milestones/M0021-algebraic-data-and-exhaustiveness.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `Test Engineer`, then `Implementer`

## Objective

Report `non_exhaustive_match` on the subject of an otherwise valid bootstrap
`when` that omits one or more declared variants.

## Authority Extract

- `docs/SPEC.md`, “ADR-0033: Bootstrap Sealed Sums And Exhaustive Match”.
- `docs/adr/ADR-0033-bootstrap-sealed-sums-and-exhaustive-match.md`,
  “Decision” and “Diagnostics And Recovery”.
- `crates/compiler/src/name_resolution.rs`: enum variant identities, resolved
  subjects, resolved arms, and match diagnostics.
- `crates/compiler/tests/name_resolution.rs`: M0021 parser-backed resolution
  tests.

## Scope

- Coverage of declared no-payload variants for one valid `when` subject.
- Wildcard coverage.
- Suppression of missing-coverage diagnostics when that `when` already has an
  invalid subject, unknown arm, or duplicate-arm diagnostic.

## Out Of Scope

- Diagnostic rendering of missing variant names, payload patterns, nullable
  coverage, result typing, and match lowering.

## Required Tests Before Implementation

- A complete qualified-variant match has no coverage diagnostic.
- A wildcard match has no coverage diagnostic.
- A match missing a declared variant diagnoses on its subject.
- A match with a prior unknown or duplicate-arm diagnostic does not add a
  non-exhaustive diagnostic.

## Acceptance Criteria

- [x] Tests fail before coverage analysis exists.
- [x] Every declared variant or one wildcard is exhaustive.
- [x] Missing coverage diagnoses with `non_exhaustive_match` on the subject.
- [x] Invalid matches do not produce an additional missing-coverage diagnostic.

## Execution Log

- 2026-07-11 agent=Main phase=task-created result=pass evidence=ADR-0033 finite coverage requirement isolated after arm provenance. handoff=Test-Engineer
- 2026-07-11 agent=Main phase=test-first result=pending evidence=parser-backed complete, wildcard, missing, and duplicate coverage regression added. handoff=Implementer
- 2026-07-11 agent=Main phase=test-first result=fail evidence=coverage analyzer and non-exhaustive diagnostic were absent. handoff=Implementer
- 2026-07-11 agent=Main phase=implementation result=pass evidence=coverage compares resolved arms against declared enum variants and suppresses cascades. handoff=Reviewer
- 2026-07-11 agent=Main phase=ordinary-tests result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets. handoff=Adversarial-Engineer
- 2026-07-11 agent=Main phase=adversarial-check result=pass evidence=docs/tasks/soundness/M0021-016-soundness.md. handoff=Reviewer
- 2026-07-11 agent=Main phase=review result=approve evidence=docs/tasks/reviews/M0021-016-review.md. handoff=Commit
