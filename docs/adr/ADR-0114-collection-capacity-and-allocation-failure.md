# ADR-0114: Collection Capacity and Allocation Failure

Status: Accepted

## Question

How do pure Neu collections expose capacity changes and allocation failure
without exposing an allocator or silently losing data?

## Decision

Collection capacity is an implementation detail observed only through ordinary
query operations such as `length` and `capacity`. `reserve`, `shrink`, and
growth performed by insertion have explicit result contracts. A request that
cannot be satisfied returns a recoverable `Result` with the operation's
documented error type; it does not partially mutate the collection, silently
discard elements, or return a nullable sentinel.

An insertion that requires growth is atomic with respect to the collection's
logical contents: it either completes with the new element present or returns
an error and leaves the prior sequence unchanged. Removal and clear do not
report allocation failure. `shrink` may retain excess capacity when the
requested reduction cannot be performed, but it must preserve all elements and
report only failures defined by its accepted signature.

No public operation accepts an allocator, deallocator, raw pointer, layout, or
capacity token. Growth policy, alignment, relocation strategy, and physical
allocation remain compiler/runtime facts. A collection may require `T` to be
movable or copyable for a particular operation; that requirement is expressed
as an explicit accepted protocol or capability bound.

## Non-goals

This ADR does not define an error payload vocabulary, memory limits, custom
allocators, persistent storage, serialization, or concurrent growth. It does
not change the generic, ownership, iterator, or protocol contracts.

## Consequences

Collections have deterministic logical behavior under allocation failure and
cannot hide partial mutation. Tests can exercise failure paths without
depending on allocator layout or a privileged stdlib entry point.

## Required follow-up

Accept this ADR (or a superseding decision), add capacity and atomic-failure
signatures to `docs/SPEC.md`, and add tests proving failed growth preserves
contents and ownership before implementing reserve/shrink or growth-sensitive
operations.

## Dependencies

This proposal depends on ADR-0108, ADR-0110, ADR-0111, ADR-0113, ADR-0063,
ADR-0064, and ADR-0016. It is based on the dependent #20 protocol branch.
