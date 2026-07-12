# ADR-0073: Bootstrap Dynamic Arrays

- Status: Accepted by the main-task Chief Architect
- Supersedes: the dynamic-array deferral statements that conflict with this
  bootstrap contract

## Decision

Neu provides a compiler-managed dynamic array type written `Array<T>`, distinct
from fixed inline `T[N]` arrays. `new T[]` creates an empty `Array<T>` and is
valid only when the element type is determined by the declared `Array<T>` type.
The bootstrap implementation accepts primitive scalar elements already
supported by the runtime ABI: `Bool`, `Int`, `Float`, and `Byte`. Strings,
nominal values, interfaces, fixed arrays, nested dynamic arrays, nullable
elements, and user-defined generic containers remain deferred to the full ABI
and nominal-array tasks.

`var` bindings permit the compiler-recognized operations `add(value)`,
`add(value, index)`, `remove(index)`, and `size()`. `val` bindings reject the
mutating operations. `add(value)` inserts at the current size; indexed add
inserts at an index in `0..size`; remove requires an index in `0..size`. The
operations return `Unit`, except `size`, which returns `Int`. Invalid indices
trap at runtime; statically known invalid indices diagnose before lowering.

Dynamic arrays are move-only compiler-managed values. The compiler owns their
storage and exposes no capacity, pointer, allocator, deallocator, layout, FFI,
or standard-library API. The initial host-linking representation is an opaque
pointer to a private length/capacity/data header. Initial capacity is private
and growth is compiler-managed; reaching an unsupported target capability or
allocation failure traps deterministically. Element destruction and full
reallocation cleanup remain part of the follow-up ownership/ABI hardening
tasks before non-scalar elements are admitted.

The source surface does not add indexing, slices, iterators, or structural
mutation during iteration. Fixed-array syntax and semantics remain governed by
ADR-0063 and ADR-0072.

## Consequences

- `Array<T>` is a distinct structural runtime type keyed by its element type.
- Dynamic array calls are compiler-recognized operations and do not use general
  method dispatch.
- Primitive dynamic-array operations can reach host and host-linking lowering;
  richer element and ownership behavior is explicitly deferred.

## Dependencies

ADR-0063, ADR-0072, ADR-0041, ADR-0055, ADR-0059, ADR-0062, ADR-0064, and the
host-linking runtime contracts.
