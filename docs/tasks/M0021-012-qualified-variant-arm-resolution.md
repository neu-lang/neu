# Task: M0021-012 Qualified Variant Arm Resolution

## Task Metadata

- Task ID: `M0021-012`
- Milestone: `M0021`
- Milestone File: `docs/milestones/M0021-algebraic-data-and-exhaustiveness.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `Test Engineer`, then `Implementer`

## Objective

Resolve each exact `Enum.Variant` arm against the enum identity already
resolved for its `when` subject.

## Authority Extract

- `docs/SPEC.md`, “ADR-0033: Bootstrap Sealed Sums And Exhaustive Match”.
- `docs/adr/ADR-0033-bootstrap-sealed-sums-and-exhaustive-match.md`,
  “Decision” and “Diagnostics And Recovery”.
- `crates/compiler/src/name_resolution.rs`: match analysis and enum variant
  identity records.
- `crates/compiler/src/parser.rs`: qualified case-pattern metadata.

## Scope

- Resolve exact same-enum variant arms.
- Report `unknown_match_variant` on a pattern whose enum or variant does not
  match the subject enum.

## Out Of Scope

- Duplicate arm checks, wildcard duplication, exhaustiveness, arm result
  typing, and all payload/destructuring semantics.

## Required Tests Before Implementation

- A qualified declared variant resolves to its variant identity.
- Unknown variant and a variant from another enum diagnose on the pattern.

## Acceptance Criteria

- [x] Tests fail before arm resolution exists.
- [x] Valid arms retain their exact enum variant node identity.
- [x] No duplicate or coverage diagnostic is added.

## Execution Log

- 2026-07-11 agent=Main phase=test-first result=fail evidence=arm-resolution API was absent. handoff=Implementer
- 2026-07-11 agent=Main phase=implementation result=pass evidence=exact same-enum arms resolve; other enums diagnose. handoff=Reviewer
- 2026-07-11 agent=Main phase=ordinary-tests result=pass evidence=focused test, formatter, strict clippy, and workspace tests passed. handoff=Adversarial-Engineer
- 2026-07-11 agent=Main phase=adversarial-check result=pass evidence=cross-enum arms diagnose without coverage semantics. handoff=Reviewer
- 2026-07-11 agent=Main phase=review result=approve evidence=ADR-0033 arm scope and validation verified. handoff=none
