# Ambiguity Report: M0006 Lexical Grammar

## Metadata

- Report ID: `M0006-LEXICAL-GRAMMAR`
- Related Task: `M0006-001`
- Related Milestone: `M0006`
- Filed By: `Language Lawyer`
- Date: `2026-07-09`
- Status: `open`
- Required Owner: `Language Designer`

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

- [x] Language Lawyer determines whether existing text resolves it.
- [x] Language Designer drafted non-authoritative ADR proposal and ownership review.
- [x] Adversarial Engineer reviewed soundness risk.
- [x] Diagnostics Engineer reviewed diagnostic consequences.
- [x] Simplicity Guardian reviewed complexity.
- [ ] Chief Architect approves final resolution.

## Temporary Rule

No implementation may proceed on concrete lexical behavior until the source of truth is updated or the ambiguity is ruled non-blocking by Chief Architect. M0006 may add inert metadata fixtures only.

## Resolution

- Decision:
  - unresolved
- Source of truth updated:
  - none
- Date resolved:
  - unresolved
