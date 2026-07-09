# ADR-0010: Type System Shape

## Question

What is the core type system?

## Competing Designs

- Nominal types with interfaces/protocols.
- Structural typing.
- Trait/typeclass-style constraints.
- Hybrid nominal data types with structural function types.

## Trade-offs

Nominal typing supports stable APIs and clear diagnostics.

Structural typing is flexible but can make errors indirect.

Traits/typeclasses are powerful for zero-cost abstraction but may feel less Kotlin-like.

Hybrids can work well but need careful boundaries.

## Recommended Choice

Nominal user-defined types, interfaces/protocols for behavior, and generic constraints capable of static dispatch where required.

## Downstream Consequences

- ABI and module boundaries can be reasoned about.
- Generic specialization and dynamic dispatch rules must be specified.
- Extension methods and protocol conformance need coherence rules.

## Dependencies

- ADR-0005
- ADR-0016
- ADR-0017

