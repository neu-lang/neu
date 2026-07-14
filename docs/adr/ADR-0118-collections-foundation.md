# ADR-0118: Pure-Neu collections foundation

Status: Accepted

## Decision

The collections package is implemented in pure Neu and is consumed through the
ordinary manifest and package-import mechanisms. The first compiler-compatible
slice provides value-level `Vector<T>`, `Slice<T>`, and `Deque<T>` shapes with
length and emptiness operations and same-file native tests.

These initial representations are algebraic values, not hidden compiler or
runtime objects. The compiler-owned `Array<T>` primitive now provides
storage-backed allocation and mutation for scalar and generic-function element
contexts; collection-level embedding, capacity policy, borrowing views,
hashing, ordering, and iterator adapters remain deferred until their core
ownership and protocol contracts are accepted. No public allocator, I/O, FFI,
or compiler-private ABI is introduced by this package.
