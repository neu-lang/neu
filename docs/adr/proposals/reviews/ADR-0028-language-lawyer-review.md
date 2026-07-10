# ADR-0028 main-task language review Review

## Metadata

- Proposal: `ADR-0028`
- Milestone: `M0019`
- main-task review: `main-task language review`
- Date: `2026-07-10`
- Decision: `request revision before acceptance`

## Inputs Read

- `docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md`
- `docs/ambiguities/M0019-nullability-and-flow-typing.md`
- `docs/milestones/M0019-nullability-and-flow-typing.md`
- `docs/SPEC.md`
- `docs/adr/ADR-0006-nullability-and-absence.md`
- `docs/adr/ADR-0011-flow-typing-and-smart-casts.md`
- `docs/adr/ADR-0013-mutability-model.md`
- `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- `docs/adr/ADR-0027-type-checking-core.md`

## Findings

No rejection findings. The proposal chooses a defensible narrow subset for M0019, but it needs revision before acceptance.

## Required Revisions

- Define branch region boundaries precisely enough that implementers can determine where each refinement starts and ends.
- State whether `if` conditions with unsupported binary expression typing use a flow-specific condition recognizer or require ADR-0027 binary expression support first.
- Define how refined expression types are represented when the underlying binding retains its original nullable type.
- State whether refinement applies only to expression uses or also to assignment compatibility checks inside the branch.
- Clarify whether shadowing of the refined local binding inside a nested block ends or hides the refinement.
- Clarify how duplicate or ambiguous local bindings reported by M0016 affect refinement eligibility.

## Compliance Notes

The proposal does not invent accepted semantics because it remains a draft and explicitly says implementation may not depend on it.

The proposal correctly preserves ADR-0027 nullable assignment compatibility and does not reopen accepted M0018 behavior.

## Decision

Request revision before acceptance. M0019 implementation must not proceed from ADR-0028 until the branch region boundaries, expression-form authority, and refined output shape are made concrete in an accepted ADR.
