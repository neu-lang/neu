# ADR-0021 Chief Architect Decision

Decision: pending

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0021-lexical-grammar.md`
- Related blocker: `docs/ambiguities/M0006-lexical-grammar.md`
- Related milestone: `docs/milestones/M0007-lexer-implementation.md`

## Current Decision

No acceptance yet.

The draft lexical grammar proposal is not accepted source of truth. It remains a planning artifact under `docs/adr/proposals/` and must not be used to implement lexer behavior.

## Completed Review Dependencies

- Language Designer ownership review.
- Adversarial Engineer soundness review.
- Diagnostics Engineer review.
- Simplicity Guardian review.

## Remaining Acceptance Blockers

- Concrete accepted lexical grammar.
- Explicit accepted token classes.
- Explicit accepted lexical error categories.
- Explicit diagnostic source span rules for lexical errors.
- Explicit Unicode identifier policy or explicit deferral.
- Explicit comment nesting behavior.
- Explicit string literal and escape behavior.
- Explicit numeric literal and overflow responsibility.
- Final Chief Architect approval through accepted ADR or `docs/SPEC.md` update.

## Consequences

- `docs/ambiguities/M0006-lexical-grammar.md` remains open.
- M0007 lexer implementation remains blocked.
- Concrete lexer source fixtures remain out of scope.
- Parser milestones may not assume token spellings from the draft.
