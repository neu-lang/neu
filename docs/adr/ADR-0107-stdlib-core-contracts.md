# ADR-0107: Foundational Standard-Library Contracts

Status: Proposed

## Question

Which language contracts must be accepted before `stdlib/core` can be
implemented as ordinary Neu source, and which APIs must remain deferred?

## Decision

`stdlib/core` is a pure Neu library. It has no compiler-only privilege, implicit
prelude, runtime ABI, allocator entry point, or special dependency path. A
project consumes it through the ordinary manifest and dependency-qualified
import mechanisms.

The first implementable core surface is limited to nominal, closed data types
and functions using the compiler's accepted generic syntax. Generic instances
are identified by their declaring nominal type and ordered concrete arguments;
type parameters are invariant; type arguments are explicit at generic calls;
and specialization is deterministic. Generic declarations may require named
capabilities, but a capability is satisfied only by an accepted concrete type
fact or an explicitly declared bound. There is no structural inference,
variance, wildcard, higher-kinded, associated-type, or implicit-prelude rule.

`Option<T>`, `Result<T, E>`, and `Ordering` are ordinary public enums. Their
constructors and operations preserve affine ownership and use explicit
borrowing. `Option<T>` represents presence or absence without implicit null
conversion. `Result<T, E>` represents success or recoverable failure without
implicit propagation or exception semantics. `Ordering` has exactly the
variants `Less`, `Equal`, and `Greater`.

The initial capability vocabulary is deliberately small: `Copy`, `Send`, and
`Share` are compiler-recognized facts; `Clone`, `Default`, `Eq`, `Ord`, and
`Hash` are library protocols only until their dispatch and bound-checking rules
are accepted. A generic API must not claim a bound whose satisfaction cannot be
checked by the compiler.

The following remain outside this ADR and must not be implemented in
`stdlib/core` yet: panic/abort/unreachable behavior, `assert` and
`debug_assert`, error-propagation syntax, numeric conversion policy, public
allocation/deallocation, formatting, Unicode normalization, and any public
`String` representation. Each requires a separate accepted decision covering
ownership, diagnostics, failure behavior, and ABI boundaries.

## Consequences

This proposal turns the broad `stdlib/core` issue into a sequence of reviewable
language and implementation changes. A core implementation may begin only
after the generic type-checking and capability facts above are accepted and
tested. Collection work must depend on the resulting core contracts and may
not define its own generic or ownership rules.

## Required follow-up

Before implementation starts, accept this ADR (or a superseding decision), add
generic substitution and bound diagnostics to `docs/SPEC.md`, and add focused
negative tests for unsatisfied and unsupported bounds. Then split the remaining
core APIs into separate decisions for error propagation, panic behavior,
numeric conversion, and string/allocation boundaries.

## Dependencies

This proposal refines ADR-0016, ADR-0006, ADR-0007, and ADR-0064 without
changing their accepted text. It depends on ADR-0001, ADR-0002, ADR-0010, and
ADR-0014.
