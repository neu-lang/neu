# ADR-0009: Async Suspension And Borrowing

## Question

What may be borrowed across suspension points?

## Competing Designs

- Forbid mutable borrows across suspension.
- Allow borrows across suspension if task-local and proven safe.
- Require explicit pinned async frames.
- Heap-allocate all async frames with compiler-managed lifetimes.

## Trade-offs

Forbidding mutable borrows is simple but restrictive.

Proven-safe borrowing preserves expressiveness but complicates analysis.

Pinning is precise but exposes advanced concepts.

Heap allocation harms predictability and moves toward runtime management.

## Recommended Choice

Allow borrows across suspension only when the compiler proves the suspended frame cannot be concurrently accessed or outlive borrowed data; require explicit annotations for advanced cases.

## Downstream Consequences

- Async functions have stricter lifetime rules than synchronous functions.
- Diagnostics must explain suspension-related borrow extension.
- Library design must avoid hidden task escape.

## Dependencies

- ADR-0002
- ADR-0003
- ADR-0008
- ADR-0014

