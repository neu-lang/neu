# ADR-0111: Recoverable Errors and Panic Boundaries

Status: Proposed

## Question

How should foundational libraries distinguish recoverable failures from
unrecoverable programmer faults without giving `stdlib/core` hidden runtime
privileges?

## Decision

Recoverable operation failures are ordinary values. A function that can fail
as part of its documented result returns `Result<T, E>` (or `Option<T>` when
absence is the complete outcome); it does not trap, return a nullable sentinel,
or encode failure in an undocumented integer. `Err(E)` is a normal value with
the ownership and `Send`/`Share` properties of `E`. Constructing, inspecting,
mapping, and returning a result use ordinary Neu functions and pattern
matching.

There is no implicit error propagation syntax in this phase. A caller must
match, return, or otherwise handle the `Result` explicitly. A function may
return an error unchanged when its declared error type matches; conversion
between error types requires an explicit library operation accepted by the
type checker. `Option<T>` is reserved for absence and must not silently absorb
an `Err` value.

Panic and abort are reserved for unrecoverable programmer or runtime faults
whose continuation would violate a language invariant. They are not the
failure path for checked indexing, parsing, conversion, or other expected
operation outcomes. The compiler/runtime reports the fault using the existing
diagnostic and termination boundary; the core library does not expose an
allocator, process, or platform escape hatch.

`assert`, `debug_assert`, `unreachable`, `todo`, `unimplemented`, and explicit
panic/abort helpers remain separate APIs. Each must specify evaluation,
diagnostic text, source span, optimization behavior, and termination outcome
before it is added to `stdlib/core`.

## Non-goals

This ADR does not define a `?`/`try` operator, exception handlers, stack
unwinding, panic recovery, error formatting, allocation failure, or the public
representation of `String`. It does not make `Clone`, `Eq`, `Ord`, or `Hash`
compiler-recognized capabilities.

## Consequences

Core and collections APIs can expose checked failures without depending on
special compiler intrinsics. Tests can distinguish returned `Err` values from
diagnosed unrecoverable faults. APIs that need panic text, allocation failure,
or error conversion remain blocked until their own contracts are accepted.

## Required follow-up

Accept this ADR (or a superseding decision), add the explicit-result and
panic-boundary rules to `docs/SPEC.md`, and add negative tests proving that
checked operations return results while invalid language invariants diagnose
and terminate. Then implement only the core helpers covered by the accepted
surface.

## Dependencies

This proposal refines ADR-0007 and depends on ADR-0107, ADR-0109, ADR-0010,
ADR-0027, and ADR-0064. It does not revise accepted ADR text.
