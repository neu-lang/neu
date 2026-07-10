# ADR-0021 main task Decision

Decision: approved

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0021-lexical-grammar.md`
- Related blocker: `docs/ambiguities/M0006-lexical-grammar.md`
- Related milestone: `docs/milestones/M0007-lexer-implementation.md`

## Current Decision

Accepted `docs/adr/ADR-0021-lexical-grammar.md` as source of truth for the bootstrap lexical grammar.

The draft lexical grammar proposal remains historical context under `docs/adr/proposals/`. Lexer behavior must be based on the accepted ADR, not the draft proposal.

## Completed Review Dependencies

- main-task semantic design ownership review.
- main-task adversarial check soundness review.
- main-task diagnostics check review.
- main-task simplicity check review.

## Resolved Acceptance Blockers

- Concrete accepted lexical grammar.
- Explicit accepted token classes.
- Explicit accepted lexical error categories.
- Explicit diagnostic source span rules for lexical errors.
- Explicit Unicode identifier policy or explicit deferral.
- Explicit comment nesting behavior.
- Explicit string literal and escape behavior.
- Explicit numeric literal and overflow responsibility.
- Final main task approval through accepted ADR or `docs/SPEC.md` update.

## Consequences

- `docs/ambiguities/M0006-lexical-grammar.md` is resolved by accepted ADR-0021.
- M0007 may proceed to concrete lexer fixtures and implementation tasks.
- Parser milestones may reference token spellings from accepted ADR-0021.
- Parser grammar, precedence, and recovery remain outside ADR-0021.
