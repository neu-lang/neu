# ADR-0014: Thread Safety And Data-Race Freedom

## Question

How does the language prevent data races at compile time?

## Competing Designs

- Send/share marker capabilities.
- Actor isolation only.
- Ownership transfer between tasks only.
- Global lock-based safety conventions.

## Trade-offs

Send/share capabilities compose with ownership and generic APIs.

Actor isolation is strong but not sufficient for all systems code.

Ownership transfer is simple but limits shared concurrency.

Lock conventions are not compile-time safety.

## Recommended Choice

Compile-time send/share capabilities, derived where sound and explicitly declared where necessary; shared mutable state requires safe synchronization abstractions.

## Downstream Consequences

- Types must declare whether they can cross thread/task boundaries.
- Captures in concurrent tasks require capability checks.
- Unsafe primitives must be isolated behind audited abstractions.

## Dependencies

- ADR-0001
- ADR-0002
- ADR-0008
- ADR-0013

