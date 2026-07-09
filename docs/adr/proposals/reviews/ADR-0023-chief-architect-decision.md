# ADR-0023 Chief Architect Decision

Decision: approved

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0023-type-and-generic-syntax.md`
- Related ambiguity: `docs/ambiguities/M0008-type-generic-syntax.md`
- Related milestone: `docs/milestones/M0012-type-and-generic-syntax-parser.md`

## Current Decision

Accept ADR-0023 as source of truth for bootstrap type and generic syntax.

Accepted source of truth: `docs/adr/ADR-0023-type-and-generic-syntax.md`

M0012 type and generic parser fixture and implementation tasks may proceed only for ADR-0023 constructs.

## Completed Review Dependencies

- Language Lawyer review.
- Adversarial Engineer review.
- Diagnostics Engineer review.
- Simplicity Guardian review.

## Resolved Acceptance Blockers

- Concrete accepted type grammar.
- Explicit nullable marker binding rules.
- Explicit generic parameter and argument grammar.
- Explicit capability-bound syntax.
- Explicit function type grammar.
- Explicit type syntax diagnostics.
- Explicit recovery boundaries.
- Final Chief Architect approval through accepted ADR and `docs/SPEC.md` update.

## Consequences

- `docs/ambiguities/M0008-type-generic-syntax.md` is resolved.
- M0012 type and generic parser implementation may proceed for ADR-0023 constructs.
- Concrete type and generic parser fixtures may be created by follow-on M0012 tasks.
- Expression, statement, pattern, coroutine, unsafe, and deferred type forms remain blocked until accepted source of truth defines them.
