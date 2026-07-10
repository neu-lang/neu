# ADR-0048 Chief Architect Decision

## Metadata

- Proposal: `ADR-0048`
- Milestone: `M0028`
- Review: `main-task chief-architect decision`
- Date: `2026-07-11`
- Decision: `accept`

## Inputs Read

- `AGENTS.md`
- `docs/adr/ADR-0048-bootstrap-integer-constant-expressions.md`
- `docs/adr/proposals/reviews/ADR-0048-language-lawyer-review.md`
- `docs/adr/proposals/reviews/ADR-0048-diagnostics-review.md`
- `docs/adr/proposals/reviews/ADR-0048-adversarial-review.md`
- `docs/adr/proposals/reviews/ADR-0048-simplicity-review.md`
- `docs/adr/proposals/reviews/ADR-0048-spec-compliance-review.md`
- `docs/ambiguities/M0028-static-integer-constant-expressions.md`

## Decision

Accept ADR-0048 under the delegated future-ADR acceptance rule in
`AGENTS.md`. The required reviews approve the narrow literal-tree boundary.

## Acceptance Bundle

1. Add ADR-0048 as an accepted ADR.
2. Add its concise source-of-truth section to `docs/SPEC.md`.
3. Resolve `docs/ambiguities/M0028-static-integer-constant-expressions.md`.
4. Update `docs/tasks/M0028-005-static-integer-diagnostics.md` with the
   accepted-resolution evidence.

## Handoff

Main task may implement M0028 static integer diagnostics against ADR-0048.
