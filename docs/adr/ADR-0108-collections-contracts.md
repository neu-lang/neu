# ADR-0108: Generic Collections Contracts

Status: Proposed

## Question

Which contracts must govern `stdlib/collections` once the foundational core
types and generic capability checks are available?

## Decision

`stdlib/collections` is a pure Neu dependency built on `stdlib/core`. It has no
compiler privilege, implicit prelude, allocator entry point, runtime ABI, or
special import path. Collection types are ordinary invariant nominal generic
types and must use the ownership, borrowing, nullability, and capability rules
accepted for core.

The first collection contract covers dynamic sequences and ordered views only:
`Vector<T>`, immutable `Slice<T>`, and a consuming/shared-borrowing
`Iterator<T>`. Each operation documents whether it consumes, shared-borrows, or
mutably borrows the receiver and element values. Indexing and removal report a
recoverable `Result` or use an explicitly accepted failure contract; they must
not silently trap or invent nullable sentinels. Mutation while an iterator is
live is rejected unless the ownership rules explicitly establish exclusivity.

Collection iteration is deterministic for sequence and ordered-view types.
Hash-based collections, specialized structures, allocation-failure behavior,
formatting, serialization, and randomization remain outside this proposal.
`HashMap`, `HashSet`, `BTreeMap`, `BTreeSet`, heaps, persistent vectors, and
interval maps require the equality, ordering, hashing, allocation, and iterator
contracts to be accepted before implementation.

## Consequences

The broad collections issue is split into reviewable phases. A collection
implementation may begin only after the core contract is accepted and the
iterator/result failure semantics are specified. No collection may define a
second generic bound vocabulary or expose public allocation primitives.

## Required follow-up

Accept this ADR or a superseding decision, then add explicit iterator protocol,
borrow invalidation, bounds-failure, allocation-failure, and complexity rules
to `docs/SPEC.md`. Add negative tests for mutation during live borrows and for
unsupported element capabilities before implementing collection code.

## Dependencies

This proposal depends on ADR-0107, ADR-0001, ADR-0002, ADR-0014, and
ADR-0016. It is sequenced after issue #19 and must not be treated as a
stdlib-specific compiler exception.
