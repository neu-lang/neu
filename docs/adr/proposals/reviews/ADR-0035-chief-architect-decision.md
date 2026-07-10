# ADR-0035 Chief Architect Decision

## Metadata

- Proposal: `ADR-0035`
- Milestone: `M0022`
- Review: `main-task chief-architect decision`
- Date: `2026-07-11`
- Decision: `accepted by delegated Chief Architect authority`

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

ADR-0035 is accepted as the resolution for the M0022 value-category blocker
under delegated Chief Architect authority. The accepted source-of-truth bundle
must exist before implementation relies on the semantics.

The proposal is the right architectural shape for M0022 because it creates a
testable ownership side-table and use-after-move diagnostic while avoiding
premature decisions about constructors, calls, returns, destructors, partial
moves, branch joins, borrow checking, coroutine frames, FFI, generic
copyability, or ABI layout.

## Acceptance Bundle

Apply one atomic source-of-truth update:

1. Create `docs/adr/ADR-0035-bootstrap-ownership-and-move-analysis.md` with
   `Status: Accepted`, removing draft-only non-authority framing.
2. Add a concise `ADR-0035: Bootstrap Ownership And Move Analysis` section to
   `docs/SPEC.md`.
3. Mark `docs/ambiguities/M0022-ownership-value-categories.md` resolved.
4. Update `docs/tasks/M0022-001-ownership-value-category-blocker.md` with the
   accepted-resolution evidence.

## Handoff

The accepted source-of-truth bundle is required before M0022 implementation
tasks may rely on ADR-0035.

## Future ADR Delegation

By project-owner delegation, future ADRs may be accepted by the main-task Chief
Architect decision once required reviews are complete. No separate owner
acceptance message is required for ADR-0036 and later unless the Chief
Architect explicitly escalates the decision back to the owner.
