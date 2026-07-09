# ADR-0021 Language Designer Review

Decision: request-revision-before-acceptance

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0021-lexical-grammar.md`
- Related blocker: `docs/ambiguities/M0006-lexical-grammar.md`
- Related milestone: `docs/milestones/M0007-lexer-implementation.md`

## Ownership Finding

The proposal direction is compatible with the project's Kotlin-like ergonomics requirement because it recommends a small custom grammar rather than treating Kotlin as inherited authority. That direction is appropriate for a compiler bootstrap, but the current proposal does not contain enough language-owned semantic detail to accept.

## Required Revisions Before Acceptance

The accepted ADR or spec update must define:

- token classes emitted by the initial lexer
- keyword policy, including fixed keywords, contextual keywords, and reserved future words
- identifier policy, including Unicode acceptance or explicit deferral
- whitespace and newline treatment
- line comment syntax
- block comment syntax and nesting behavior
- literal policy for integers, strings, escapes, suffixes, and unsupported literal forms
- operator and delimiter set for the initial parser milestones
- diagnostic-facing lexical rules for invalid characters, malformed literals, unterminated comments, unterminated strings, and unsupported forms
- source span rules for valid tokens and invalid lexical regions
- explicit deferral list for lexical forms that are intentionally not part of the bootstrap compiler

## Required Non-Goals

The accepted revision should not:

- adopt Kotlin's full lexical grammar wholesale
- accept syntax merely because it appears in Kotlin
- define parser precedence or expression grammar
- define type semantics for literals beyond the lexical boundary
- add future-facing token abstractions that are not needed by the accepted grammar

## Non-Authority Finding

This review is not accepted source of truth. It records Language Designer ownership feedback only.

Agents must not implement lexer behavior from this review or from the draft proposal. Lexer implementation remains blocked until the Chief Architect accepts a concrete lexical grammar into `docs/SPEC.md` or accepted `docs/adr/`.
