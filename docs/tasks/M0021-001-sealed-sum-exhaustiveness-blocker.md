# Task: M0021-001 Sealed Sum And Exhaustiveness Blocker

## Task Metadata

- Task ID: `M0021-001`
- Milestone: `M0021`
- Milestone File: `docs/milestones/M0021-algebraic-data-and-exhaustiveness.md`
- Specification: `docs/SPEC.md`
- Status: `blocked`
- Owner: `Language Designer`

## Objective

Record the missing accepted semantics that block M0021 implementation.

## Authority Extract

- `docs/SPEC.md`, “ADR-0012: Pattern Matching And Algebraic Data”,
  “ADR-0022: Declaration Syntax”, and “ADR-0024: Expression Statement And
  Pattern Syntax”.
- `docs/adr/ADR-0012-pattern-matching-and-algebraic-data.md`.
- `docs/adr/ADR-0022-declaration-syntax.md`, “Enum Declarations”.
- `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`.
- `docs/ambiguities/M0021-sealed-sum-exhaustiveness.md`.

## Blocker

Accepted ADRs choose the direction but defer the variants, match form, sealing
scope, coverage algorithm, and diagnostics needed to implement M0021.

## Required Resolution

Language Designer must provide an accepted ADR or SPEC revision defining a
small parser-backed sealed-sum and match subset, exhaustiveness behavior, and
diagnostics. No implementation or tests may infer Kotlin behavior.

## Execution Log

- 2026-07-10 agent=Main phase=blocker-recorded result=blocked evidence=accepted source defers enum variants and match/when syntax. handoff=Language-Designer
