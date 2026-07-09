# Token Model Planning

Status: Planning artifact for M0006

This document records token-model status only. It does not define lexical grammar and does not authorize lexer implementation.

## Source Of Truth

Token categories must be justified by:

- `docs/SPEC.md`
- `docs/adr/`
- an accepted future grammar or lexical ADR

No token category may be implemented from Kotlin precedent alone.

## Status Labels

- specified: enough source-of-truth text exists to create lexer fixtures.
- blocked: a required lexical rule is missing or ambiguous.
- deferred: the category belongs to a later approved milestone.

## Token Category Status

| Category | Status | Authority | Notes |
| --- | --- | --- | --- |
| Whitespace handling | blocked | `docs/SPEC.md` is silent | Needed before lexer implementation. |
| Line comments | blocked | `docs/SPEC.md` is silent | Do not assume Kotlin comment syntax. |
| Block comments | blocked | `docs/SPEC.md` is silent | Nesting behavior is unspecified. |
| Identifiers | blocked | `docs/SPEC.md` is silent | Unicode and keyword interaction are unspecified. |
| Keywords | blocked | `docs/SPEC.md` names concepts but not lexical spellings | Do not infer keyword set from Kotlin or Rust. |
| Integer literals | blocked | `docs/SPEC.md` is silent | Bases, separators, suffixes, and overflow behavior are unspecified. |
| String literals | blocked | `docs/SPEC.md` is silent | Escape rules and raw strings are unspecified. |
| Operators | blocked | `docs/SPEC.md` is silent | Operator spellings and precedence are unspecified. |
| Delimiters | blocked | `docs/SPEC.md` is silent | Braces, brackets, parentheses, commas, dots, and semicolons are unspecified. |
| End-of-file marker | deferred | Lexer architecture detail | Can be represented after grammar authority exists. |
| Error token | deferred | Diagnostic infrastructure exists, lexical errors do not | Add after lexical error rules exist. |

## Fixture Policy

M0006 fixtures may only be inert metadata fixtures. They must cite `docs/SPEC.md`, `docs/adr/`, or this milestone and must not include source text, expected tokens, token streams, keyword spellings, identifiers, literals, operators, or delimiters.

Concrete lexer fixtures are blocked until detailed lexical grammar is accepted.

