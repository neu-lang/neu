# ADR-0027 Spec Compliance Review

## Metadata

- ADR: `ADR-0027`
- Milestone: `M0018`
- Reviewer: `Spec Compliance Auditor`
- Date: `2026-07-10`
- Decision: `request revision before acceptance`

## Source Of Truth Review

ADR-0027 is not accepted source of truth. `docs/SPEC.md` does not contain an ADR-0027 section, and `docs/adr/ADR-0027-type-checking-core.md` does not exist.

The proposal correctly cites accepted dependencies, but it must not be used to implement type checking until promoted by Chief Architect and summarized in `docs/SPEC.md`.

The accepted ADR must reconcile the M0018 ambiguity report and update it to resolved only after source-of-truth text exists.

## Required Revisions

- Keep the proposal clearly marked as non-authoritative until acceptance.
- Add an accepted ADR only after required reviews are addressed.
- Update `docs/SPEC.md` only as part of acceptance.
- Resolve `docs/ambiguities/M0018-type-checking-core.md` only after acceptance.
- Preserve explicit deferrals for unaccepted typing rules.

## Boundary

This review is not accepted source of truth. Implementation must continue to compare behavior against `docs/SPEC.md` and accepted ADRs, not this proposal.
