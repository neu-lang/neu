# ADR-0002: Borrowing Semantics

## Question

How are references permitted without compromising safety?

## Competing Designs

- Rust-like shared-or-exclusive borrowing.
- Capability-based borrowing with named permissions.
- Region-only borrowing without explicit exclusivity.
- Implicit borrow inference with minimal surface syntax.

## Trade-offs

Shared-or-exclusive borrowing is proven, but can feel rigid.

Capability borrowing is expressive, but more complex to teach and diagnose.

Region-only borrowing is simpler, but weaker for mutation safety.

Heavy inference improves ergonomics, but may obscure why code is rejected.

## Recommended Choice

Shared immutable borrows or one exclusive mutable borrow, with Kotlin-like surface syntax and strong inference.

## Downstream Consequences

- Mutation must be tied to exclusive access.
- Collections and iterators must be designed around borrow splitting or alternative traversal rules.
- Diagnostics must explain aliasing conflicts in user terms, not compiler internals.

## Dependencies

- ADR-0001
- ADR-0003
- ADR-0011
- ADR-0015

