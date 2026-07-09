# ADR-0004: Destruction And Resource Finalization

## Question

How are resources released without garbage collection or manual frees?

## Competing Designs

- Deterministic destructors on scope exit.
- Explicit resource blocks only.
- Compiler-inserted reference counting.
- Linear disposal obligations.

## Trade-offs

Deterministic destruction is predictable and familiar to systems programmers.

Resource blocks are clear, but verbose and incomplete for general ownership.

Reference counting changes the cost model.

Linear disposal is very safe, but burdens common code.

## Recommended Choice

Deterministic destruction tied to ownership, with structured resource scopes as ergonomic sugar where useful.

## Downstream Consequences

- Drop order must be specified.
- Partial moves and destructor interaction must be defined.
- Async cancellation must run destructors predictably.

## Dependencies

- ADR-0001
- ADR-0008
- ADR-0009
- ADR-0014

