# Task: M0021-013 Duplicate Enum Variant Diagnostics

## Task Metadata

- Task ID: `M0021-013`
- Milestone: `M0021`
- Milestone File: `docs/milestones/M0021-algebraic-data-and-exhaustiveness.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `Test Engineer`, then `Implementer`

## Objective

Detect repeated no-payload enum variant names within one declared bootstrap
enum and report `duplicate_enum_variant` on the repeated variant.

## Authority Extract

- `docs/SPEC.md`, “ADR-0033: Bootstrap Sealed Sums And Exhaustive Match”.
- `docs/adr/ADR-0033-bootstrap-sealed-sums-and-exhaustive-match.md`,
  “Diagnostics And Recovery”.
- `crates/compiler/src/name_resolution.rs`: enum variant identity index.

## Scope

- Diagnose a repeated declared variant within the same enum only.

## Out Of Scope

- Match-arm duplicate checks, wildcard checks, unknown variants, missing
  variants, and exhaustiveness.

## Required Tests Before Implementation

- Repeated variants in one enum report `duplicate_enum_variant` on the later
  variant.
- Like-named variants in different enums remain valid.

## Acceptance Criteria

- [x] Tests fail before duplicate enum analysis exists.
- [x] Primary location is the repeated variant node.
- [x] The first variant remains the canonical identity.

## Execution Log

- 2026-07-11 agent=Main phase=test-first result=pending evidence=duplicate enum analyzer is absent. handoff=Implementer
- 2026-07-11 agent=Main phase=test-first result=fail evidence=duplicate enum analyzer was absent. handoff=Implementer
- 2026-07-11 agent=Main phase=implementation result=pass evidence=later duplicate variants diagnose while the first remains canonical. handoff=Reviewer
- 2026-07-11 agent=Main phase=ordinary-tests result=pass evidence=focused test, formatter, strict clippy, and workspace tests passed. handoff=Adversarial-Engineer
- 2026-07-11 agent=Main phase=adversarial-check result=pass evidence=duplicate detection is enum-scoped and preserves canonical first variants. handoff=Reviewer
- 2026-07-11 agent=Main phase=review result=approve evidence=ADR-0033 scope and validation verified. handoff=none
