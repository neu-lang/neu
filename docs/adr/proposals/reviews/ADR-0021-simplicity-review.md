# ADR-0021 Simplicity Review

Decision: pass-with-required-resolution

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0021-lexical-grammar.md`
- Related blocker: `docs/ambiguities/M0006-lexical-grammar.md`

## Simplicity Findings

The recommended draft direction is acceptable for the bootstrap compiler because it chooses a small Kotlin-like custom lexical grammar and rejects adopting Kotlin wholesale. That keeps the language aligned with Kotlin-like ergonomics without importing Kotlin's full lexical surface as accidental semantics.

The accepted version should stay narrow:

- prefer a minimal keyword set required by current parser milestones
- avoid contextual keywords unless a later parser milestone proves the need
- avoid rich numeric literal suffixes until type checking has accepted authority
- defer Unicode display-column policy unless Unicode identifiers are accepted now
- define only operators and delimiters required by near-term grammar milestones
- keep string literal forms minimal until interpolation or raw strings are explicitly approved

## Required Resolution Before Acceptance

- Separate required bootstrap tokens from future lexical extensions.
- Mark deferred forms as rejected or blocked, not unspecified.
- Avoid adding abstraction layers to support token families that are not yet accepted.
- Keep lexer fixture scope limited to accepted tokens and accepted lexical errors.

## Non-Authority Finding

This review supports the proposal direction but does not accept the lexical grammar. No implementation may rely on this draft.
