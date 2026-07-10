# Ambiguity Report: M0008 Expression, Statement, And Pattern Syntax

## Metadata

- Report ID: `M0008-EXPRESSION-STATEMENT-PATTERN-SYNTAX`
- Related Task: `M0008-001`
- Related Milestone: `M0008`
- Filed By: `main-task language review`
- Date: `2026-07-09`
- Status: `resolved`
- Required Owner: `main-task semantic design`
- Blocking milestone: `M0013`

## Ambiguous Or Missing Authority

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0007-error-handling.md`
  - `docs/adr/ADR-0008-structured-concurrency-semantics.md`
  - `docs/adr/ADR-0009-async-suspension-and-borrowing.md`
  - `docs/adr/ADR-0011-flow-typing-and-smart-casts.md`
  - `docs/adr/ADR-0012-pattern-matching-and-algebraic-data.md`
  - `docs/adr/ADR-0018-unsafe-ffi-and-trust-boundaries.md`
  - `docs/adr/ADR-0021-lexical-grammar.md`

## Exact Ambiguous Text Or Missing Rule

```text
Accepted ADRs define high-level behavior for errors, concurrency, suspension, flow typing, pattern matching, and unsafe boundaries. ADR-0021 defines token spellings only. No accepted source defines expression grammar, precedence, associativity, statement grammar, block grammar, pattern grammar, unsafe block syntax, coroutine syntax, or recovery boundaries.
```

## Competing Interpretations

1. Infer Kotlin expression, statement, and `when` syntax.
2. Infer Rust-like block-expression and pattern syntax.
3. Define a smaller custom grammar in a future accepted ADR.
4. Defer concrete expression, statement, and pattern parser fixtures until grammar authority exists.

## Why guessing Is Unsafe

- Expression precedence and associativity affect program meaning.
- Pattern syntax affects exhaustiveness and smart-cast behavior.
- Statement and block grammar affect ownership scopes and destruction points.
- Coroutine and unsafe syntax affect safety boundaries.
- Parser recovery behavior can become user-visible diagnostics.

## Affected Work

- Tasks blocked:
  - M0013 expression, statement, and pattern parser tasks.
- Milestones affected:
  - M0013
  - M0018
  - M0019
  - M0022
  - M0025
- Tests blocked:
  - Concrete expression parser fixtures.
  - Concrete statement parser fixtures.
  - Concrete pattern parser fixtures.
- Implementation areas blocked:
  - Expression AST.
  - Statement AST.
  - Pattern AST.
  - Parser recovery for expressions and statements.

## Recommended Resolution Path

- [x] main-task semantic design drafts expression, statement, and pattern syntax ADR or `docs/SPEC.md` revision.
- [x] main-task language review audits consistency with safety and diagnostics ADRs.
- [x] main-task adversarial check reviews ownership scope, coroutine, and unsafe-boundary risks.
- [x] main-task diagnostics check reviews parser recovery and diagnostic obligations.
- [x] main task approves final source-of-truth update.

## Temporary Rule

Parser implementation may accept only the concrete expression, statement, and pattern syntax defined by ADR-0024 until future accepted source of truth extends it.

## Resolution

- Decision:
  - Accept a small Kotlin-like custom expression, statement, block, and pattern grammar for the bootstrap compiler.
- Source of truth updated:
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/SPEC.md`
- Date resolved:
  - 2026-07-10
