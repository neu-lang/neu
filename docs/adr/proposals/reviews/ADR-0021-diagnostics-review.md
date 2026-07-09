# ADR-0021 Diagnostics Review

Decision: pass-with-required-resolution

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0021-lexical-grammar.md`
- Diagnostic authority: `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- Related blocker: `docs/ambiguities/M0006-lexical-grammar.md`

## Diagnostic Requirements

The accepted lexical grammar must define diagnostic behavior as part of the semantics, not as later implementation polish. Per ADR-0015, lexical diagnostics are user-visible language behavior.

Acceptance requires explicit lexical error categories for at least:

- unknown character or byte sequence
- unterminated string literal
- invalid string escape
- unterminated block comment
- malformed numeric literal
- unsupported or malformed identifier character
- ambiguous operator or delimiter sequence, if such sequences exist

Each category must define source spans precisely enough for fixtures and snapshots:

- span start and end for the smallest invalid source range
- whether the diagnostic points at the delimiter, body, suffix, escape sequence, or full token
- whether recovery resumes after one byte, one scalar value, one delimiter, or the whole malformed token
- whether follow-on diagnostics are suppressed for the same malformed token

## Required Resolution Before Acceptance

- Add lexical error categories to the accepted ADR or `docs/SPEC.md`.
- Tie each category to source spans from M0005.
- State which diagnostics are required for M0007 and which are explicitly deferred.
- Avoid compiler-internal jargon in diagnostic wording requirements.

## Non-Authority Finding

The draft proposal is suitable for review planning. It does not yet define accepted diagnostics and is not a basis for lexer diagnostic snapshots.
