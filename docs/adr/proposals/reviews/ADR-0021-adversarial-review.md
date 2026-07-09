# ADR-0021 Adversarial Review

Decision: pass-with-required-resolution

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0021-lexical-grammar.md`
- Status reviewed: draft proposal, not accepted source of truth
- Related blocker: `docs/ambiguities/M0006-lexical-grammar.md`

## Soundness Concerns

The proposal identifies the right decision surface, but it is not sufficient for lexer implementation until the accepted version makes the following rules explicit:

- identifier Unicode policy, including whether identifiers are ASCII-only, Unicode-aware, normalized, or deliberately deferred
- comment nesting behavior, including malformed nested or unterminated block comments
- string escapes, including unknown escapes, unterminated strings, line continuations, and Unicode escapes if any
- integer literal overflow responsibility, including whether overflow is a lexical diagnostic or deferred to later semantic analysis
- contextual keyword behavior, if any, because lexer and parser authority must not disagree
- invalid token recovery boundaries, because later parser diagnostics must not reinterpret bad lexical spans

## Required Resolution Before Acceptance

- Define every token class that the initial lexer may emit.
- Define every lexical error category that the initial lexer may report.
- Define whether invalid source produces error tokens, diagnostics with skipped bytes, or both.
- Define source span boundaries for invalid literals, comments, identifiers, and operators.
- Keep all unapproved lexical forms rejected or blocked rather than accepted by convention.

## Non-Authority Finding

This review does not approve lexical semantics. The proposal remains non-authoritative and cannot be used to implement a lexer until accepted through the project decision process.
