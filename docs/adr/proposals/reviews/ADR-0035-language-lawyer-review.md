# ADR-0035 Language Lawyer Review

## Metadata

- Proposal: `ADR-0035`
- Milestone: `M0022`
- Review: `main-task language-lawyer review`
- Date: `2026-07-11`
- Decision: `approve for owner acceptance`

## Inputs Read

- `docs/adr/proposals/ADR-0035-bootstrap-ownership-and-move-analysis.md`
- `docs/ambiguities/M0022-ownership-value-categories.md`
- `docs/SPEC.md`, ADR-0001, ADR-0005, ADR-0015, ADR-0027, ADR-0032,
  ADR-0033, and ADR-0034 summaries
- `docs/milestones/M0022-ownership-and-move-analysis.md`

## Review

The proposal answers the ambiguity with a bounded bootstrap rule instead of
deriving semantics from existing compiler behavior. It preserves ADR-0001's
single-owner affine direction and ADR-0005's primitive-copy/user-move split,
while explicitly narrowing the undefined primitive scalar set and move-site
table for M0022.

The text correctly avoids deciding destructor execution, borrow checking,
partial moves, calls, returns, closures, coroutine frames, user-declared copy,
generic copyability, ABI layout, and target representation. These exclusions
are stated as deferrals, so later ADRs can supersede the bootstrap subset
without contradicting it.

`String` as move-only is defensible because ADR-0005 says primitive scalar
types copy, and ADR-0027 says primitive identities have no ABI/layout meaning.
The proposal should be accepted only if the owner agrees that this is the
intended bootstrap rule.

## Open Questions

- None blocking acceptance.

## Handoff

Chief Architect for final owner-acceptance decision.
