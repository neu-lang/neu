# Task: M0021-005 Qualified Case Pattern Metadata

## Task Metadata

- Task ID: `M0021-005`
- Milestone: `M0021`
- Milestone File: `docs/milestones/M0021-algebraic-data-and-exhaustiveness.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `Test Engineer`, then `Implementer`

## Objective

Record the enum and variant identifier metadata for exact ADR-0033 qualified
case patterns, without resolving or diagnosing them.

## Authority Extract

- `docs/SPEC.md`, “ADR-0033: Bootstrap Sealed Sums And Exhaustive Match”.
- `docs/adr/ADR-0033-bootstrap-sealed-sums-and-exhaustive-match.md`, “Decision”.
- `crates/newlang/src/parser.rs`: pattern and `when` parsing.
- Validation: `cargo test -p newlang --test parser m0021_qualified_case_pattern`;
  `cargo fmt --all --check`; `git diff --check`.

## Scope

- Record source-order metadata only for exact `identifier . identifier` patterns.

## Out Of Scope

- Variant resolution, coverage, duplicate diagnostics, and changes to parser recovery.

## Required Tests Before Implementation

- A `when` arm records enum and variant names with exact spans.
- Longer qualified and payload-shaped patterns do not record bootstrap metadata.

## Acceptance Criteria

- [x] Tests fail before metadata APIs exist.
- [x] Exact qualified patterns retain both identifiers and pattern node identity.
- [x] No match semantics or diagnostics are added.

## Execution Log

- 2026-07-10 agent=Main phase=create-task result=pass evidence=metadata only. handoff=Test-Engineer
- 2026-07-10 agent=Main phase=test-first result=fail evidence=parser output lacked qualified case pattern metadata. handoff=Implementer
- 2026-07-10 agent=Main phase=implementation result=pass evidence=exact two-identifier patterns record names and spans; other qualified forms do not. handoff=Reviewer
- 2026-07-10 agent=Main phase=ordinary-tests result=pass evidence=focused test, formatter, and strict clippy passed. handoff=Adversarial-Engineer
- 2026-07-10 agent=Main phase=adversarial-check result=pass evidence=non-bootstrap qualified shapes cannot receive bootstrap metadata. handoff=Reviewer
- 2026-07-10 agent=Main phase=review result=approve evidence=ADR-0033 parser-only scope and full validation verified. handoff=none
