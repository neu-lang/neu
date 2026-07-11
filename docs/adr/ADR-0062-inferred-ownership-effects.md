# ADR-0062: Inferred Ownership Effects

Status: Accepted

## Question

How can Neu infer ownership effects for calls and local operations without
exposing reference, lifetime, or move syntax?

## Decision

Neu infers ownership effects from function bodies and call sites. Effects are
compiler metadata, not source syntax. Users do not write `&`, `&mut`, explicit
dereferences, lifetime annotations, or `move`.

The effect categories are:

- `Read`: a temporary shared use that leaves the source binding available;
- `Mutate`: an exclusive use that consumes a move-only source binding;
- `Consume`: ownership transfer into storage, a consuming parameter, or another
  operation that makes the source unavailable;
- `Store`: ownership retained by a destination beyond the current operation;
  and
- `ReturnOwned`: a returned value that gives the caller a new owned value.

Copyable values follow ADR-0005 and remain available after by-value calls.
Move-only values become unavailable after `Mutate`, `Consume`, or `Store`
effects. Both `val` and `var` may be consumed. Only `var` may receive an
atomic consume-and-rebind operation such as `x = f(x)` when `f` consumes the
old value and returns a compatible owned value.

Function contracts record, per parameter, the inferred effect, source
projections, implicit borrow regions, conditional path states, and whether a
returned value carries ownership. A contract is inferred from the function
body; it is not supplied by an annotation.

Conditional effects are path-sensitive. Binding states are `Available`,
`Consumed`, or `MaybeConsumed`. A binding is usable after a control-flow join
only when every reachable incoming path proves it available. Loops use a
conservative fixed-point join and reject uses that are not proven available on
every reachable iteration path.

Methods and fields, when their syntax and runtime representation are accepted,
use receiver and field projections. A mutating projection creates an
exclusive effect on that projection and its owning value. Closures and
coroutine suspension effects remain deferred.

Separate compilation requires exported effect metadata. Missing, stale, or
incompatible metadata is a compile-time error; the compiler never guesses a
callee effect. The current bootstrap source driver only validates effects for
source forms whose type and call metadata are already accepted. Move-only
nominal runtime values, methods, fields, closures, and coroutines remain
deferred until their existing frontend and backend boundaries are accepted.

## Diagnostics

The effect checker reports:

- `use_after_consumption` when a consumed binding is reused;
- `possible_use_after_consumption` when a `MaybeConsumed` binding is reused;
- `invalid_consuming_call` when a call requires consumption that the argument
  cannot satisfy;
- `non_rebindable_consumed_value` when a consumed `val` is used as a rebind
  destination; and
- `missing_effect_metadata` when a required separate-compilation contract is
  unavailable or stale.

Diagnostics identify the consuming operation, inferred effect, source binding,
and relevant path or projection spans. Invalid effects are reported before
HIR, MIR, Cranelift, object emission, or linking.

## Intermediate And ABI Contract

HIR preserves function effect contracts, per-call effect selection, binding
state transitions, implicit borrow regions, projection identity, consumption
origins, returned-ownership facts, and source spans. MIR preserves those facts
through calls, returns, local stores, branches, loops, and atomic rebinds.

The ABI remains ordinary Neu value passing under the existing internal calling
convention. Effects do not add hidden reference parameters, lifetime ABI,
stable public layout, FFI behavior, or runtime ownership services.

## Deferrals And Supersession

This ADR extends ADR-0001, ADR-0002, ADR-0035, and ADR-0036 with call and
path-sensitive effect facts. It does not add source-level borrowing syntax or
supersede the existing no-GC, no-manual-free, thread-safety, coroutine, array,
slice, string, allocation, or FFI deferrals.

## Dependencies

- ADR-0001
- ADR-0002
- ADR-0003
- ADR-0004
- ADR-0005
- ADR-0015
- ADR-0024
- ADR-0027
- ADR-0035
- ADR-0036
- ADR-0041
- ADR-0044
- ADR-0045
- ADR-0046
- ADR-0060
