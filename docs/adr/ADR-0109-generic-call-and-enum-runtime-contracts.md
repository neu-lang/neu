# ADR-0109: Generic Calls and Enum Runtime Contracts

Status: Proposed

## Question

What concrete call-site and runtime rules are required before the generic
`stdlib/core` enums can be implemented without adding compiler-only behavior?

## Decision

Generic calls use explicit type arguments in the first implementation phase.
The compiler resolves a call by pairing those arguments, in declaration order,
with the function's generic parameters. The argument count must match exactly;
missing, extra, or out-of-order arguments are diagnostics. Generic parameters
are invariant and nominal: a specialization is identified by the declaring
function or type and its ordered concrete arguments. There is no inference,
variance conversion, wildcard argument, or implicit conversion at a generic
call site.

Call-site specialization happens after the concrete argument types are known
and before lowering. The compiler substitutes those types through parameter,
return, and bound expressions, then checks every declared capability bound. A
failed bound is reported at the generic call and must not be deferred to code
generation. Only compiler-recognized `Copy`, `Send`, and `Share` facts are
valid bounds in this phase; library protocols do not become compiler facts by
name alone.

`Option<T>`, `Result<T, E>`, and `Ordering` are ordinary closed public enums.
Each concrete generic instantiation is a distinct nominal type. Constructors
carry or release payloads according to the ordinary affine ownership rules;
pattern matching must handle the declared variants and cannot inspect an
unstable representation. `Option<T>` has `Some(T)` and `None`; `Result<T, E>`
has `Ok(T)` and `Err(E)`; `Ordering` has `Less`, `Equal`, and `Greater`.

The representation of a generic enum is an implementation detail of the
compiler and runtime. No field offsets, discriminant encoding, niche layout,
allocation strategy, or cross-module generic ABI is part of this contract.
Specialization must therefore be deterministic within one compilation, while
separate compilation and public ABI guarantees remain deferred.

## Non-goals

This ADR does not define generic type inference, overload resolution,
error-propagation syntax, panic or abort behavior, allocation failure,
formatting, or the public representation of `String`. Those decisions remain
separate prerequisites for the corresponding standard-library APIs.

## Consequences

The compiler may implement checked generic call specialization and ordinary
enum constructors/matches without granting `stdlib/core` privileged syntax or
an ABI promise. Diagnostics and tests can target the call site, making bound
and arity failures deterministic. Collection work may depend on these rules
but must still specify iterator, borrowing, and bounds-failure behavior.

## Required follow-up

Accept this ADR (or a superseding decision), then add the call-site and enum
rules to `docs/SPEC.md`. Add positive and negative compiler tests for explicit
specialization, argument-count errors, bound failures, and exhaustive enum
matching before implementing the corresponding core APIs.

## Dependencies

This proposal refines ADR-0107 and depends on ADR-0001, ADR-0002, ADR-0014,
ADR-0016, and ADR-0007. It does not revise accepted ADR text.
