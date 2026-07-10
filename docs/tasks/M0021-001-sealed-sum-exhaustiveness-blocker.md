# Task: M0021-001 Sealed Sum And Exhaustiveness Blocker

## Task Metadata

- Task ID: `M0021-001`
- Milestone: `M0021`
- Milestone File: `docs/milestones/M0021-algebraic-data-and-exhaustiveness.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
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

ADR-0033 is accepted and defines the small parser-backed enum/`when` subset,
exhaustiveness behavior, and diagnostics. Payloads and destructuring remain
deferred.

## Execution Log

- 2026-07-10 agent=Main phase=blocker-recorded result=blocked evidence=accepted source defers enum variants and match/when syntax. handoff=Language-Designer
- 2026-07-10 agent=Main phase=resolution-accepted result=pass evidence=user accepted ADR-0033; source of truth updated. handoff=Task-Decomposer
