# ADR-0008: Structured Concurrency Semantics

## Question

What guarantees does concurrent execution provide?

## Competing Designs

- Kotlin-like structured concurrency with lexical task scopes.
- Go-like detached goroutines by default.
- Explicit thread spawning only.
- Actor-only concurrency.

## Trade-offs

Structured concurrency gives clear cancellation, lifetime, and ownership boundaries.

Detached concurrency is simple to start but hard to reason about.

Explicit threads are predictable but too low-level.

Actor-only avoids data races but may be too restrictive.

## Recommended Choice

Structured concurrency as the default concurrency model; detached work must be explicit and constrained.

## Downstream Consequences

- Child tasks cannot outlive their scope unless ownership and cancellation semantics allow it.
- Borrowing across suspension and task boundaries must be specified.
- Cancellation becomes part of resource safety.

## Dependencies

- ADR-0001
- ADR-0002
- ADR-0004
- ADR-0009

