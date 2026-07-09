# ADR-0024 Chief Architect Decision

Decision: approved

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0024-expression-statement-pattern-syntax.md`
- Related ambiguity: `docs/ambiguities/M0008-expression-statement-pattern-syntax.md`
- Related milestone: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`

## Current Decision

Accept ADR-0024 as source of truth for bootstrap expression, statement, block, and pattern syntax.

Accepted source of truth: `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`

M0013 body parser fixture and implementation tasks may proceed only for ADR-0024 constructs.

## Completed Review Dependencies

- Language Lawyer review.
- Adversarial Engineer review.
- Diagnostics Engineer review.
- Simplicity Guardian review.

## Resolved Acceptance Blockers

- Concrete expression grammar.
- Operator precedence and associativity.
- Concrete statement grammar.
- Concrete block grammar.
- Concrete pattern grammar.
- Explicit unsafe block deferral.
- Explicit coroutine syntax deferral.
- Explicit parser diagnostics.
- Explicit parser recovery boundaries.
- Final Chief Architect approval through accepted ADR and `docs/SPEC.md` update.

## Consequences

- `docs/ambiguities/M0008-expression-statement-pattern-syntax.md` is resolved.
- M0013 parser fixtures may be created for ADR-0024 constructs.
- M0013 parser implementation may proceed for ADR-0024 constructs.
- Coroutine syntax, unsafe block syntax, match or `when`, loops, and other ADR-0024 deferrals remain out of scope.
