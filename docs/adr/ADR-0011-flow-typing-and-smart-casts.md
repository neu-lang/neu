# ADR-0011: Flow Typing And Smart Casts

## Question

How much Kotlin-style flow-sensitive typing is allowed?

## Competing Designs

- Full smart casts for nullability, type tests, and initialization.
- Nullability-only smart casts.
- No smart casts; require explicit unwrapping.
- Smart casts only for immutable local values.

## Trade-offs

Full smart casts improve ergonomics but interact with mutation and borrowing.

Nullability-only is simpler but less expressive.

No smart casts conflicts with Kotlin-like goals.

Immutable-only smart casts are safe but need clear rules.

## Recommended Choice

Flow-sensitive smart casts for immutable or exclusively borrowed values; mutation invalidates refinements.

## Downstream Consequences

- Borrow checker and flow analyzer must cooperate.
- Diagnostics must explain why a refinement was invalidated.
- Pattern matching semantics depend on this decision.

## Dependencies

- ADR-0002
- ADR-0006
- ADR-0012

