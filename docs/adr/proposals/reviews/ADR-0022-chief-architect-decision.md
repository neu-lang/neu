# ADR-0022 Chief Architect Decision

Decision: approved

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0022-declaration-syntax.md`
- Related ambiguity: `docs/ambiguities/M0008-declaration-syntax.md`
- Related milestone: `docs/milestones/M0011-declaration-parser.md`

## Current Decision

Accepted source of truth: `docs/adr/ADR-0022-declaration-syntax.md`

The concrete declaration grammar is approved for the bootstrap compiler. The draft proposal remains historical review context under `docs/adr/proposals/`; parser behavior must cite the accepted ADR, not the draft.

## Completed Review Dependencies

- Language Lawyer review.
- Diagnostics Engineer review.
- Simplicity Guardian review.

## Resolved Acceptance Blockers

- Concrete accepted declaration grammar.
- Explicit package and import ordering rules.
- Explicit visibility and modifier rules.
- Explicit declaration header grammar.
- Explicit member declaration rules and deferrals.
- Explicit declaration diagnostics.
- Final Chief Architect approval through accepted ADR and `docs/SPEC.md` update.

## Consequences

- `docs/ambiguities/M0008-declaration-syntax.md` is resolved.
- M0011 declaration parser fixture and implementation tasks may proceed for ADR-0022 constructs only.
- Type, generic, expression, statement, and pattern parser tasks remain blocked on their own accepted syntax authority.
