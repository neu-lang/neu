# ADR-0092: Array Element Iteration

## Status

Accepted.

## Decision

Neu supports `for (value in array)` and its unparenthesized equivalent for
fixed `T[N]` and compiler-managed `Array<T>` values. The array expression is
evaluated exactly once, then elements are visited in ascending index order,
from zero through the last element. Empty arrays execute no body iterations.

The loop binding is an immutable compiler-inferred read-only binding. Copyable
elements are copied into the binding. Move-only elements are implicitly read
through the existing shared-borrow ownership effect and are not consumed from
the array. The array remains usable after the loop. The binding cannot be
assigned or rebound. Existing `break` and `continue` semantics apply.

Indexed writes during iteration remain subject to existing `var`, ownership,
and projection-overlap rules. Structural dynamic-array mutation during the
iteration is rejected, so no iterator invalidation semantics are introduced.
Nested arrays, accepted element types, parameters, returns, cleanup, source
spans, and target-pack behavior retain their existing contracts. HIR and MIR
preserve the array target, fixed/dynamic identity, element projection, loop
binding, cleanup, and back-edge facts rather than treating the operation as a
plain integer range.

## Consequences

The parser distinguishes an array target from the existing `start..end` range
form. Type checking validates the target and gives the loop binding its element
type. Lowering evaluates the target once and uses bounded index-based access
for fixed arrays and the compiler-private length/storage contract for dynamic
arrays. No iterator object, slice, view, generator, public pointer, or new
borrow syntax is added.

## Dependencies

ADR-0060, ADR-0062, ADR-0063, ADR-0072, ADR-0073, ADR-0091.

## Deferred

Reverse iteration, custom steps, labels, comprehensions, parallel iteration,
user-defined iterators, slices, views, generators, structural mutation during
iteration, and new array or ownership syntax remain deferred.
