# Task: M0021-011 When Subject Analysis

## Task Metadata

- Task ID: `M0021-011`
- Milestone: `M0021`
- Milestone File: `docs/milestones/M0021-algebraic-data-and-exhaustiveness.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `Test Engineer`, then `Implementer`

## Objective

Create the ADR-0033 match-analysis report and validate each `when` subject
against the ADR-0034 resolved enum parameter identity.

## Authority Extract

- `docs/SPEC.md`, “ADR-0033: Bootstrap Sealed Sums And Exhaustive Match” and
  “ADR-0034: Bootstrap Enum Subject Typing”.
- `docs/adr/ADR-0033-bootstrap-sealed-sums-and-exhaustive-match.md`,
  “Diagnostics And Recovery”.
- `docs/adr/ADR-0034-bootstrap-enum-subject-typing.md`, “Decision”.
- `crates/newlang/src/name_resolution.rs`: local reference and enum-parameter
  identity records.

## Scope

- Record a resolved enum identity for valid bare parameter subjects.
- Report `invalid_match_subject` on every other `when` subject.

## Out Of Scope

- Variant lookup, duplicate arms, wildcard handling, exhaustiveness, arm type
  checking, and all ownership/flow behavior.

## Required Tests Before Implementation

- A bound enum parameter subject resolves to its enum declaration.
- A non-enum parameter and a non-name subject report `invalid_match_subject`.

## Acceptance Criteria

- [x] Tests fail before subject analysis exists.
- [x] Diagnostics use the subject node as primary location.
- [x] No arm or coverage semantic is added.

## Execution Log

- 2026-07-11 agent=Main phase=test-first result=pending evidence=subject analysis API has not been implemented. handoff=Implementer
- 2026-07-11 agent=Main phase=implementation result=pass evidence=resolved enum parameters become subject records; other subjects diagnose. handoff=Reviewer
- 2026-07-11 agent=Main phase=ordinary-tests result=pass evidence=focused test, formatter, strict clippy, and workspace tests passed. handoff=Adversarial-Engineer
- 2026-07-11 agent=Main phase=adversarial-check result=pass evidence=resolved bindings prevent spelling-only subject identity. handoff=Reviewer
- 2026-07-11 agent=Main phase=review result=approve evidence=ADR-0033/0034 subject scope and validation verified. handoff=none
