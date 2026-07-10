# M0021 Sealed Sum And Exhaustiveness Blocker

## Exact Source Text

ADR-0012 selects sealed sums and exhaustive matching at a high level. ADR-0022
states that enum variant grammar is deferred and that `enum` is only the
bootstrap sealed-sum declaration keyword. ADR-0024 accepts individual pattern
forms but explicitly defers `match` or `when` syntax.

No accepted source defines variant declarations, the scope that seals a sum,
match-arm syntax, pattern-to-variant resolution, exhaustiveness rules, nullable
coverage, smart-cast interaction, or diagnostic behavior for missing cases.

## Competing Interpretations

1. Treat current empty `enum` declaration shells and standalone pattern nodes
   as sufficient for exhaustiveness checking.
2. Infer Kotlin-style `when` and enum-variant behavior.
3. Defer M0021 semantic checking until a concrete ADR defines the model.

## Decision Required

Only option 3 is currently authorized. Options 1 and 2 would invent language
semantics. A Language Designer ADR must define the algebraic data and matching
subset before implementation or tests encode accepted/rejected programs.

## Affected Work

- `docs/milestones/M0021-algebraic-data-and-exhaustiveness.md`
- `docs/adr/ADR-0012-pattern-matching-and-algebraic-data.md`
- `docs/adr/ADR-0022-declaration-syntax.md`
- `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`

## Resolution

Resolved by accepted ADR-0033 for the bootstrap no-payload enum and
expression-level `when` subset. Other algebraic-data and match forms remain
deferred.
