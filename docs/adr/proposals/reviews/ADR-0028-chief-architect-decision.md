# ADR-0028 Chief Architect Decision

## Metadata

- Proposal: `ADR-0028`
- Milestone: `M0019`
- Decider: `Chief Architect`
- Date: `2026-07-10`
- Decision: `request revision before acceptance`

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

Decision: request revision before acceptance.

ADR-0028 is directionally approved as a narrow M0019 design, but it is not accepted source of truth.

Implementation may not proceed against ADR-0028 until accepted by a later task that moves the ADR into the accepted ADR set, updates `docs/SPEC.md`, and resolves `docs/ambiguities/M0019-nullability-and-flow-typing.md`.

## Required Revision Scope

- Define branch region boundaries.
- Define flow-specific condition recognition or explicitly require prerequisite binary expression type checking.
- Define refined expression type output shape.
- Define shadowing and nested scope behavior.
- Define diagnostic primary spans, recovery actions, source-of-truth citations, safe suggestion policies, and stable rule identifiers.
- Preserve explicit deferrals for members, calls, aliases, coroutine suspension, unsafe, FFI, generics, patterns, and exclusive-borrow refinements.

## Implementation Authority

Implementation may not proceed against ADR-0028 until accepted.

The only currently accepted M0019-relevant behavior remains ADR-0027 nullable assignment compatibility.

## Next Step

Create a revision task for ADR-0028 that incorporates the required review findings into a concrete acceptance-ready draft.
