# ADR-0028 Chief Architect Decision

## Metadata

- Proposal: `ADR-0028`
- Milestone: `M0019`
- Decider: `Chief Architect`
- Date: `2026-07-10`
- Decision: `approved`

## Inputs Read

- `docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md`
- `docs/adr/proposals/reviews/ADR-0028-language-lawyer-review.md`
- `docs/adr/proposals/reviews/ADR-0028-diagnostics-review.md`
- `docs/adr/proposals/reviews/ADR-0028-adversarial-review.md`
- `docs/adr/proposals/reviews/ADR-0028-spec-compliance-review.md`
- `docs/adr/proposals/reviews/ADR-0028-simplicity-review.md`
- `docs/ambiguities/M0019-nullability-and-flow-typing.md`
- `docs/milestones/M0019-nullability-and-flow-typing.md`

## Decision

Decision: approved.

ADR-0028 is accepted as the M0019 nullability and flow-typing source of truth.

M0019 ambiguity is resolved by `docs/adr/ADR-0028-nullability-and-flow-typing.md` and `docs/SPEC.md`.

## Required Revision Scope

- Define branch region boundaries.
- Define flow-specific condition recognition or explicitly require prerequisite binary expression type checking.
- Define refined expression type output shape.
- Define shadowing and nested scope behavior.
- Define diagnostic primary spans, recovery actions, source-of-truth citations, safe suggestion policies, and stable rule identifiers.
- Preserve explicit deferrals for members, calls, aliases, coroutine suspension, unsafe, FFI, generics, patterns, and exclusive-borrow refinements.

## Implementation Authority

Implementation may proceed only against the accepted ADR-0028 model.

ADR-0028 is accepted. ADR-0027 nullable assignment compatibility remains part of the accepted input model.

## Next Step

Create implementation tasks for the accepted M0019 subset.
