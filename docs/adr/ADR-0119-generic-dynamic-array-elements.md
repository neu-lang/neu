# ADR-0119: Generic dynamic-array elements

Status: Accepted

## Decision

The compiler-managed `Array<T>` primitive may use an in-scope generic type
parameter as its element type in generic functions. `new T[]`, `add`, `remove`,
size, iteration, and indexed operations retain the existing move-only storage,
mutation, bounds-trap, and deterministic-destruction rules. Generic element
values use the compiler-private opaque scalar representation already used for
nominal runtime values; this does not expose an allocator or public ABI.

Generic collection fields, nominal element destruction, slices, and
specialization inference remain deferred until their ownership metadata can be
represented without ambiguity.
