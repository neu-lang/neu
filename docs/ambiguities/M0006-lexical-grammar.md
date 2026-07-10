# Ambiguity Report: M0006 Lexical Grammar

## Metadata

- Report ID: `M0006-LEXICAL-GRAMMAR`
- Related Task: `M0006-001`
- Related Milestone: `M0006`
- Filed By: `main-task language review`
- Date: `2026-07-09`
- Status: `resolved`
- Required Owner: `main-task semantic design`

## Ambiguous Or Missing Authority

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/`
- Milestone:
  - `docs/milestones/M0006-token-model-and-lexer-fixtures.md`

## Exact Ambiguous Text Or Missing Rule

```text
Detailed lexical grammar is missing. docs/SPEC.md says the language has Kotlin-like syntax and records semantic ADR decisions, but it does not define whitespace, comments, identifiers, keywords, literals, operators, delimiters, or lexical error rules.
```

## Competing Interpretations

1. Treat Kotlin lexical grammar as the default.
2. Define a smaller custom lexical grammar in a future ADR.
3. Defer all concrete lexer fixtures until a grammar authority document exists.

## Why guessing Is Unsafe

- guessing from Kotlin would create language syntax without an accepted decision.
- Lexer fixtures would become de facto semantics.
- M0007 lexer implementation would accept or reject programs without source-of-truth authority.
- Diagnostics could encode invalid token expectations.

## Affected Work

- Tasks blocked:
  - `M0007` lexer implementation tasks that need concrete source text.
- Milestones affected:
  - `M0007`
  - `M0008`
- Tests blocked:
  - Concrete positive and negative lexer fixtures.
- Implementation areas blocked:
  - Lexer tokenization rules.
  - Lexical diagnostics.

## Recommended Resolution Path

- [x] main-task language review determines whether existing text resolves it.
- [x] main-task semantic design drafted non-authoritative ADR proposal and ownership review.
- [x] main-task adversarial check reviewed soundness risk.
- [x] main-task diagnostics check reviewed diagnostic consequences.
- [x] main-task simplicity check reviewed complexity.
- [x] main task approves final resolution.

## Temporary Rule

No implementation may proceed on concrete lexical behavior unless it is backed by `docs/adr/ADR-0021-lexical-grammar.md` or a later accepted source-of-truth update.

## Resolution

- Decision:
  - Accepted `docs/adr/ADR-0021-lexical-grammar.md` as source of truth for bootstrap lexical grammar.
- Source of truth updated:
  - `docs/adr/ADR-0021-lexical-grammar.md`
  - `docs/SPEC.md`
- Date resolved:
  - 2026-07-09
