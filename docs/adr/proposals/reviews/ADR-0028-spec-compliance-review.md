# ADR-0028 Spec Compliance Review

## Metadata

- Proposal: `ADR-0028`
- Milestone: `M0019`
- main-task review: `main-task specification check`
- Date: `2026-07-10`
- Decision: `request revision before acceptance`

## Inputs Read

- `docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md`
- `docs/ambiguities/M0019-nullability-and-flow-typing.md`
- `docs/milestones/M0019-nullability-and-flow-typing.md`
- `docs/SPEC.md`
- accepted ADRs referenced by the proposal

## Compliance Findings

- ADR-0028 is not accepted source of truth.
- `docs/SPEC.md` does not contain ADR-0028.
- `docs/ambiguities/M0019-nullability-and-flow-typing.md` remains open.
- The proposal is consistent with ADR-0006 and ADR-0011 at a high level.
- The proposal preserves ADR-0027 nullable assignment compatibility.
- The proposal correctly does not authorize implementation.

## Required Revisions Before Acceptance

- Add accepted wording that explicitly resolves the M0019 ambiguity report.
- Define concrete source-of-truth text suitable for insertion into `docs/SPEC.md`.
- Ensure the accepted ADR depends on ADR-0027 and does not rely on parser or implementation behavior as authority.
- Confirm whether ADR-0024 `if` expression syntax is sufficient for M0019 branch refinement without accepting value typing for `if` expressions more broadly.

## Decision

Request revision before acceptance. Proposal-only scope is compliant; implementation remains blocked.
