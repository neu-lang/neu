# Ambiguity: M0021 Enum Subject Typing

## Exact Source Text

ADR-0033 requires a `when` subject to resolve to exactly one declared bootstrap
enum. ADR-0022 and `docs/SPEC.md` still defer function-parameter contents, and
ADR-0027 defines only primitive type-checking identities. ADR-0033 defines
qualified enum variants only as patterns, not as value expressions.

## Competing Interpretations

1. Infer Kotlin-like enum parameters, nominal type checking, and
   `Enum.Variant` value expressions.
2. Treat parser metadata or test-only inputs as an enum-typed subject.
3. Accept a separate narrow source subset that defines enum-typed subject
   introduction and nominal type resolution for M0021.
4. Defer executable exhaustiveness semantics until a later language decision.

## Why Guessing Is Unsafe

Each interpretation changes which programs can introduce an enum value and
which name/type-resolution diagnostics apply. The current accepted example
uses `fun code(signal: Signal)`, but no accepted grammar or type-checking rule
gives that parameter its enum type.

## Affected Authority

- `docs/SPEC.md`, ADR-0022, ADR-0027, and ADR-0033.
- `docs/adr/ADR-0022-declaration-syntax.md`.
- `docs/adr/ADR-0027-type-checking-core.md`.
- `docs/adr/ADR-0033-bootstrap-sealed-sums-and-exhaustive-match.md`.
- `docs/milestones/M0021-algebraic-data-and-exhaustiveness.md`.

## Recommended Owner

Language Designer, with Chief Architect approval.
