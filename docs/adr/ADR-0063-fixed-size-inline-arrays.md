# ADR-0063: Fixed-Size Inline Arrays

Status: Accepted

## Question

What fixed-size array semantics can Neu support end-to-end without adding
allocation, slice, string, FFI, or stable public layout semantics?

## Decision

Neu supports fixed-size inline array types written `[T; N]`. Array type
identity is structural and includes the recursively validated element type and
the length. `N` is a non-negative integer literal or the value of a named
compile-time `const Int` accepted by ADR-0061. Zero-length arrays are valid.

Array literals use `[e1, e2, ...]` and must contain exactly `N` elements. There
is no repeat initializer, default initialization, inferred length, runtime
length, or allocation syntax. Nested arrays are supported recursively.

Indexing uses `array[index]`. The index must have type `Int`; the result is an
element place that can be read as a value or written through a mutable `var`
array when the element is assignable. A statically known negative or
out-of-range index is a diagnostic. A dynamically invalid index traps and
never wraps.

`val` array bindings are immutable runtime values. `var` array bindings permit
indexed assignment. `const` remains a compile-time primitive declaration under
ADR-0061 and cannot bind an array value.

The initial executable subset supports arrays whose elements are copyable
bootstrap primitives or recursively supported arrays. Such arrays copy
element-wise. Nominal, move-only, or destructor-bearing array elements are
deferred from executable lowering, but the ownership contract reserves
recursive reverse-index destruction for the future aggregate cleanup model.

Constant-index projections have distinct ownership-effect places. Runtime
indexed projections conservatively overlap. No source borrow, slice, region,
or move syntax is introduced.

Arrays are inline local values. Same-module parameters and returns use the
internal Neu convention only; no stable public layout, FFI ABI, heap placement,
allocation API, or standard-library support is defined.

## HIR, MIR, And Backend Contract

HIR preserves structural array types, literal element order, index expressions,
element places, bounds facts, source spans, and ownership-effect projections.
MIR preserves aggregate construction, ordered initialization, local aggregate
storage, indexed loads and stores, bounds checks, trap terminators, and source
mapping. The backend lowers supported arrays as compiler-internal inline
aggregates through the existing host-linking pipeline.

## Deferrals

Dynamic arrays, slices, strings, allocation APIs, heap-backed arrays, public
layout guarantees, FFI, standard-library support, nominal move-only arrays,
destructor execution, and coroutine-array interactions remain deferred.

## Supersession And Dependencies

This ADR supersedes the array and indexing deferrals in ADR-0024 and ADR-0042
for the accepted fixed-size subset. It refines the runtime-type boundary in
ADR-0055 and the HIR, MIR, ABI, primitive, and host-linking contracts in
ADR-0044 through ADR-0046 and ADR-0100. It uses named compile-time constants
from ADR-0061 and ownership projection facts from ADR-0062.
