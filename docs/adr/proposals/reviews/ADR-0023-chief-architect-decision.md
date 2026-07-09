# ADR-0023 Chief Architect Decision

Decision: pending

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0023-type-and-generic-syntax.md`
- Related ambiguity: `docs/ambiguities/M0008-type-generic-syntax.md`
- Related milestone: `docs/milestones/M0012-type-and-generic-syntax-parser.md`

## Current Decision

No acceptance yet.

The draft type and generic syntax proposal is not accepted source of truth. It remains a planning artifact under `docs/adr/proposals/` and must not be used to implement parser behavior.

## Completed Review Dependencies

- Language Lawyer review.
- Adversarial Engineer review.
- Diagnostics Engineer review.
- Simplicity Guardian review.

## Remaining Acceptance Blockers

- Concrete accepted type grammar.
- Explicit nullable marker binding rules.
- Explicit generic parameter and argument grammar.
- Explicit capability-bound syntax.
- Explicit function type grammar.
- Explicit type syntax diagnostics.
- Explicit recovery boundaries.
- Final Chief Architect approval through accepted ADR or `docs/SPEC.md` update.

## Consequences

- `docs/ambiguities/M0008-type-generic-syntax.md` remains open.
- M0012 type and generic parser implementation remains blocked.
- Concrete type and generic parser fixtures remain out of scope.
