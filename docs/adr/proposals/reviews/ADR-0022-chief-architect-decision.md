# ADR-0022 Chief Architect Decision

Decision: pending

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0022-declaration-syntax.md`
- Related ambiguity: `docs/ambiguities/M0008-declaration-syntax.md`
- Related milestone: `docs/milestones/M0011-declaration-parser.md`

## Current Decision

No acceptance yet.

The draft declaration syntax proposal is not accepted source of truth. It remains a planning artifact under `docs/adr/proposals/` and must not be used to implement parser behavior.

## Completed Review Dependencies

- Language Lawyer review.
- Diagnostics Engineer review.
- Simplicity Guardian review.

## Remaining Acceptance Blockers

- Concrete accepted declaration grammar.
- Explicit package and import ordering rules.
- Explicit visibility and modifier rules.
- Explicit declaration header grammar.
- Explicit member declaration rules or deferrals.
- Explicit declaration diagnostics.
- Final Chief Architect approval through accepted ADR or `docs/SPEC.md` update.

## Consequences

- `docs/ambiguities/M0008-declaration-syntax.md` remains open.
- M0011 declaration parser implementation remains blocked.
- Concrete declaration parser fixtures remain out of scope.
