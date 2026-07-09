# ADR-0012: Pattern Matching And Algebraic Data

## Question

Does the language have algebraic data types and exhaustive matching?

## Competing Designs

- Sealed sum types with exhaustive matching.
- Open class hierarchies only.
- Enum variants with payloads only.
- Pattern matching as library-level destructuring.

## Trade-offs

Sealed sum types are excellent for safe domain modeling.

Open hierarchies fit OO designs but weaken exhaustiveness.

Payload enums are compact but may be less Kotlin-like.

Library destructuring cannot enforce exhaustiveness well.

## Recommended Choice

Sealed sum types with exhaustive pattern matching, integrated with smart casts.

## Downstream Consequences

- Exhaustiveness checking becomes a compiler responsibility.
- Public API evolution must define compatibility rules for sealed variants.
- Error handling and nullable modeling can use the same semantic foundation.

## Dependencies

- ADR-0006
- ADR-0007
- ADR-0011
- ADR-0017

