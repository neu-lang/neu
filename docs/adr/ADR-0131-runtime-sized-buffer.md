# ADR-0131: Runtime-Sized Buffer

Status: Accepted; replaces the built-in dynamic array of ADR-0073 and ADR-0119,
and the `T[]` spelling that ADR-0129 would have introduced

## Question

Library containers such as `ArrayList<T>` need storage whose length is chosen
at run time. The only array type in the language is fixed `T[N]` (ADR-0063),
whose length is a compile-time constant. ADR-0067 forbids an allocator
primitive. How should a library obtain runtime-sized storage?

## Decision

### The type

`Buffer<T>` is a compiler-provided class that holds a fixed number of slots,
chosen when it is allocated. It is declared in the standard library:

    internal intrinsic class Buffer<T> {
        static intrinsic func allocate(capacity: Int): Buffer<T>;
        intrinsic func capacity(): Int;
        intrinsic func swap(index: Int, value: T?): T?;
    }

- `internal` limits the name to its package (ADR-0025). User packages cannot
  name `Buffer`.
- `intrinsic` marks a body-less declaration whose implementation the compiler
  provides. It is accepted only in standard-library packages. Elsewhere it is a
  diagnostic.
- `Buffer<T>` has no source-level fields and no other operations.

### Slots

Each slot has the type `T?`. An empty slot holds `null`.

- `swap(index, value)` moves `value` into the slot and moves the previous
  contents out, so no element is copied.
- `index` must satisfy `0 <= index < capacity()`. The compiler does not
  check this in the intrinsic, which traps on violation. The library checks
  the index first and reports an error as ADR-0108 requires.
- Erasure (ADR-0118): a slot of erased type `T?` stores a presence flag and
  the element word. The flag distinguishes `null` from a present element whose
  word is zero, such as `0` or `false`.

### Ownership

- `Buffer<T>` is move-only. Copying a buffer is not permitted, regardless of
  `T`, since a copy would need a deep copy, which is deferred with clone (#62).
- Destroying a buffer destroys its non-null slots in reverse index order, and
  then releases its storage (ADR-0067).
- Allocation failure follows ADR-0114.

### Deferred

- Uninitialized slots and move-in initialization.
- Resizing a buffer in place.
- Exposing a buffer's storage as a slice.
- Element iteration over a buffer. Containers provide iteration.

## Consequences

- `ArrayList<T>` is ordinary Neu code over `Buffer<T?>`, with private fields
  (ADR-0130).
- The compiler's built-in dynamic array is removed: `new T[]`, the dynamic
  `add`, `remove`, and `size` operations, for-each over a dynamic array, and
  dynamic element writes. The spelling `Array` is no longer a type name.
  Fixed arrays `T[N]` are unchanged.
- `Vector<T>` and `Deque<T>` are rebuilt over `Buffer<T>` or dropped in
  favour of `ArrayList<T>`, as decided separately.
