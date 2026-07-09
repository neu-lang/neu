# ADR-0013: Mutability Model

## Question

Is mutability a property of bindings, values, references, or types?

## Competing Designs

- Kotlin-like mutable/immutable bindings.
- Type-level mutability.
- Borrow-level mutability only.
- Deep immutability by default.

## Trade-offs

Binding mutability is familiar but insufficient for aliasing safety alone.

Type-level mutability is powerful but can infect APIs.

Borrow-level mutability matches ownership safety.

Deep immutability is strong for concurrency but restrictive.

## Recommended Choice

Immutable bindings by default, mutable bindings explicit, and mutation authority controlled by exclusive mutable borrows.

## Downstream Consequences

- Thread safety can reason about shared immutable data.
- Interior mutability requires explicit safe abstractions.
- Smart casts and borrow rules must account for mutation authority.

## Dependencies

- ADR-0002
- ADR-0011
- ADR-0014

