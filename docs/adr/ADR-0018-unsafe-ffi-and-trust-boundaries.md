# ADR-0018: Unsafe, FFI, And Trust Boundaries

## Question

How does the language interact with unsafe operations and foreign code?

## Competing Designs

- No unsafe language surface.
- Explicit unsafe blocks/functions/modules.
- Capability-gated unsafe APIs.
- FFI through generated safe bindings only.

## Trade-offs

No unsafe surface is unrealistic for systems programming.

Explicit unsafe blocks are clear but can become too granular.

Capability-gated unsafe APIs improve auditability.

Generated bindings are ergonomic but cannot cover every low-level case.

## Recommended Choice

Explicit unsafe functions and blocks, with module-level audit boundaries and safe wrappers required for ordinary use.

## Downstream Consequences

- Unsafe code must not weaken safe-code guarantees.
- FFI nullability, ownership, lifetime, and thread guarantees must be declared.
- Diagnostics and documentation must distinguish proven safety from trusted assertions.

## Dependencies

- ADR-0001
- ADR-0003
- ADR-0006
- ADR-0014

