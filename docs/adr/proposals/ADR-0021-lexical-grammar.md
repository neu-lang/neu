# ADR-0021: Lexical Grammar

Status: Draft proposal - not accepted source of truth

## Non-Authority Notice

This file is a draft proposal only. It is not accepted language semantics, not an accepted ADR, and not a valid basis for lexer implementation.

No lexer implementation may depend on this proposal until accepted by main task and moved into the accepted ADR set or incorporated into `docs/SPEC.md`.

The active blocker remains `docs/ambiguities/M0006-lexical-grammar.md`.

## Question

What concrete lexical grammar should the language use for whitespace, comments, identifiers, keywords, literals, operators, delimiters, and lexical errors?

## Competing Designs

1. Adopt Kotlin lexical grammar directly.
2. Define a small Kotlin-like custom lexical grammar for the compiler bootstrap.
3. Define only ASCII lexical grammar initially, deferring Unicode and rich literals.
4. Continue deferring lexical grammar until broader syntax design is complete.

## Trade-offs

Adopting Kotlin lexical grammar directly maximizes familiarity, but it imports many decisions the project has not evaluated, including literal forms, escapes, soft keywords, and Unicode behavior.

A small Kotlin-like custom grammar preserves ergonomic direction while keeping the compiler bootstrap understandable. It requires deliberate decisions and may differ from user expectations where Kotlin precedent is strong.

An ASCII-only initial grammar is easiest to implement and diagnose, but may feel artificially narrow for a modern language. It also creates migration work when Unicode identifiers or richer literals are added.

Continuing to defer grammar avoids premature semantics, but blocks M0007 lexer work and all later parser milestones.

## Recommended Draft Choice

Define a small Kotlin-like custom lexical grammar for the compiler bootstrap, with explicit status for every token class and deferred Unicode display policy.

The accepted version should specify:

- whitespace handling
- line comment syntax
- block comment syntax and nesting behavior
- identifier start and continue characters
- keyword list and whether any keywords are contextual
- integer literal forms, separators, suffixes, and overflow responsibility
- string literal forms and escape rules
- operator and delimiter spellings
- lexical error categories
- source span behavior for invalid tokens

The accepted version should not rely on Kotlin, Rust, Go, Cranelift, or LLVM defaults as implicit authority.

## Downstream Consequences

- M0007 can create concrete positive and negative lexer fixtures only after acceptance.
- M0007 lexer diagnostics can be defined only for accepted lexical error categories.
- M0008 grammar authority can reference accepted token spellings instead of guessing them.
- Diagnostic snapshots must cite accepted lexical rules, not this draft.
- Unicode identifier and display-column policy may need a separate ADR if not included in the accepted lexical grammar.

## Dependencies

- `docs/SPEC.md`
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- `docs/ambiguities/M0006-lexical-grammar.md`
- main task approval
- main-task semantic design ownership review
- main-task adversarial check soundness review
- main-task diagnostics check review
- main-task simplicity check review

