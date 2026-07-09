# Token Model Planning

Status: Planning artifact updated for M0007

This document records token-model status only. Accepted lexical grammar is defined by `docs/adr/ADR-0021-lexical-grammar.md`.

## Source Of Truth

Token categories must be justified by:

- `docs/SPEC.md`
- `docs/adr/`
- `docs/adr/ADR-0021-lexical-grammar.md`

No token category may be implemented from Kotlin precedent alone.

## Status Labels

- specified: enough source-of-truth text exists to create lexer fixtures.
- blocked: a required lexical rule is missing or ambiguous.
- deferred: the category belongs to a later approved milestone.

## Token Category Status

| Category | Status | Authority | Notes |
| --- | --- | --- | --- |
| Whitespace handling | specified | ADR-0021 | Whitespace separates tokens and is not emitted. |
| Line comments | specified | ADR-0021 | `//` to line break or end of file. |
| Block comments | specified | ADR-0021 | Nested `/* ... */` comments. |
| Identifiers | specified | ADR-0021 | ASCII identifiers; Unicode identifiers are deferred. |
| Keywords | specified | ADR-0021 | Fixed reserved keyword set; no contextual keywords. |
| Integer literals | specified | ADR-0021 | Decimal, binary, hexadecimal, separators; suffixes deferred. |
| String literals | specified | ADR-0021 | Double-quoted strings with minimal escapes. |
| Operators | specified | ADR-0021 | Token spellings only; parser precedence is out of scope. |
| Delimiters | specified | ADR-0021 | Parentheses, braces, brackets, comma, and semicolon. |
| End-of-file marker | deferred | Lexer architecture detail | Can be represented after grammar authority exists. |
| Error token | specified | ADR-0021 and ADR-0015 | Lexical error categories and spans are specified. |

## Fixture Policy

M0006 fixtures remain inert metadata fixtures.

M0007 concrete lexer fixtures may include source text, expected tokens, and expected lexical diagnostics only when they cite `docs/adr/ADR-0021-lexical-grammar.md`.
