# ADR-0024 Chief Architect Decision

Decision: pending

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0024-expression-statement-pattern-syntax.md`
- Related ambiguity: `docs/ambiguities/M0008-expression-statement-pattern-syntax.md`
- Related milestone: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`

## Current Decision

No acceptance yet.

The draft expression, statement, and pattern syntax proposal is not accepted source of truth. It remains a planning artifact under `docs/adr/proposals/` and must not be used to implement parser behavior.

## Completed Review Dependencies

- Language Lawyer review.
- Adversarial Engineer review.
- Diagnostics Engineer review.
- Simplicity Guardian review.

## Remaining Acceptance Blockers

- Concrete expression grammar.
- Operator precedence and associativity.
- Concrete statement grammar.
- Concrete block grammar.
- Concrete pattern grammar.
- Explicit unsafe block inclusion or deferral.
- Explicit coroutine syntax inclusion or deferral.
- Explicit parser diagnostics.
- Explicit parser recovery boundaries.
- Final Chief Architect approval through accepted ADR or `docs/SPEC.md` update.

## Consequences

- `docs/ambiguities/M0008-expression-statement-pattern-syntax.md` remains open.
- M0013 parser fixtures remain blocked.
- M0013 parser implementation remains blocked.
- Concrete expression, statement, and pattern AST nodes remain out of scope.
