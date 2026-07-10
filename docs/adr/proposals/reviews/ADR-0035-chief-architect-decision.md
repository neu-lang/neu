# ADR-0035 Chief Architect Decision

## Metadata

- Proposal: `ADR-0035`
- Milestone: `M0022`
- Review: `main-task chief-architect decision`
- Date: `2026-07-11`
- Decision: `approve for owner acceptance`

## Inputs Read

- `docs/adr/proposals/ADR-0035-bootstrap-ownership-and-move-analysis.md`
- `docs/adr/proposals/reviews/ADR-0035-language-lawyer-review.md`
- `docs/adr/proposals/reviews/ADR-0035-adversarial-review.md`
- `docs/adr/proposals/reviews/ADR-0035-diagnostics-review.md`
- `docs/adr/proposals/reviews/ADR-0035-simplicity-review.md`
- `docs/adr/proposals/reviews/ADR-0035-spec-compliance-review.md`
- `docs/ambiguities/M0022-ownership-value-categories.md`
- `docs/milestones/M0022-ownership-and-move-analysis.md`

## Decision

ADR-0035 is approved as the proposed resolution for the M0022 value-category
blocker, subject to explicit owner acceptance. This decision does not make the
proposal authoritative and does not authorize implementation before
acceptance.

The proposal is the right architectural shape for M0022 because it creates a
testable ownership side-table and use-after-move diagnostic while avoiding
premature decisions about constructors, calls, returns, destructors, partial
moves, branch joins, borrow checking, coroutine frames, FFI, generic
copyability, or ABI layout.

## Acceptance Bundle

If accepted by the owner, apply one atomic source-of-truth update:

1. Create `docs/adr/ADR-0035-bootstrap-ownership-and-move-analysis.md` with
   `Status: Accepted`, removing draft-only non-authority framing.
2. Add a concise `ADR-0035: Bootstrap Ownership And Move Analysis` section to
   `docs/SPEC.md`.
3. Mark `docs/ambiguities/M0022-ownership-value-categories.md` resolved.
4. Update `docs/tasks/M0022-001-ownership-value-category-blocker.md` with the
   accepted-resolution evidence.

## Handoff

Owner acceptance is required before M0022 implementation tasks may begin.

## Future ADR Delegation

After the project owner's ADR-0035 acceptance, future ADRs may be accepted by
the main-task Chief Architect decision once required reviews are complete. No
separate owner acceptance message is required for ADR-0036 and later unless the
Chief Architect explicitly escalates the decision back to the owner.
