# ADR-0110: Collection Iteration and Failure Contracts

Status: Proposed

## Question

Which operational rules must `stdlib/collections` follow before sequence and
ordered-view implementations can be added on top of the core contracts?

## Decision

`Vector<T>` owns a mutable sequence. `Slice<T>` is an immutable borrowed view
whose lifetime is tied to the source sequence. `Iterator<T>` is a consuming
or explicitly shared-borrowing traversal; every constructor and operation
states which of those two modes it uses. A mutable borrow of a vector is
exclusive, and a vector cannot be mutated, moved, or destroyed while a live
slice or iterator borrow could observe it. The compiler must reject such an
operation rather than silently invalidate the view.

Sequence iteration is deterministic and visits elements in increasing index
order. `Slice<T>` preserves the corresponding source order. An iterator is
single-pass unless a future accepted protocol explicitly says otherwise;
calling `next` after exhaustion returns the protocol's terminal result and
does not restart or read past the sequence.

Indexing, removal, and other operations whose precondition can fail return an
explicit recoverable result. An out-of-bounds access must not trap, panic,
return a nullable sentinel, or use an undocumented error value. The concrete
error payload and the result-construction helpers are part of the core/error
contract and must be accepted before the public API is frozen.

The first sequence operations promise linear traversal and amortized constant
append where growth is required; indexing is constant time; removing from the
middle is linear in the number of shifted elements. These complexity promises
are API contracts, not permission to expose allocation or representation
details. Allocation failure, capacity management, parallel iteration, and
randomized ordering remain outside this ADR.

## Non-goals

This ADR does not define hash collections, persistent collections, allocator
APIs, formatting, serialization, or mutation of a collection through an
iterator. It also does not replace the generic call and enum contracts in
ADR-0109.

## Consequences

The compiler and library can test borrow invalidation and deterministic
iteration without exposing a special collections runtime. Bounds failures are
visible in the type system and can be composed once the core error contract is
accepted. Implementations must not proceed as though an out-of-bounds panic or
implicit nullable conversion were available.

## Required follow-up

Accept this ADR (or a superseding decision), add the borrow, iterator,
complexity, and failure rules to `docs/SPEC.md`, and add negative tests for
mutation during a live view plus out-of-bounds operations. Then implement the
smallest `Vector`/`Slice`/`Iterator` surface in ordinary Neu source.

## Dependencies

This proposal depends on ADR-0107, ADR-0108, ADR-0109, ADR-0001, ADR-0002,
ADR-0014, and ADR-0016. It is intentionally based on the current generic
contract branch so related work remains linear until the prerequisite PRs are
merged.
