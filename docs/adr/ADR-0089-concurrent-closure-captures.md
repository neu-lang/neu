# ADR-0089: Concurrent Closure Captures

## Status

Accepted.

## Decision

An owned closure transferred to an approved thread or structured-task boundary
requires `Send` for every moved or mutable capture. A closure shared across a
concurrently accessible boundary requires `Share` for every shared capture.
Borrowed captures never cross such a boundary; they are rejected before HIR
lowering. Mutable shared captures are rejected because no shared-mutation
primitive is accepted. Capture capability is derived from the captured type,
using ADR-0037's existing `Send` and `Share` rules.

Closure-boundary records preserve the closure boundary, capture source span,
binding, capture mode, type, required capability, and diagnostic provenance.
Structured concurrency keeps the closure within its containing task scope.
Completion and cancellation both run compiler-managed capture cleanup before
the scope is released. No detached task, scheduler API, lock, channel, public
closure layout, FFI closure ABI, or user-visible transfer syntax is added.

Suspension uses the existing borrow-across-suspension rules. A borrowed capture
that would outlive its source or enter a concurrently accessible frame is a
diagnostic; an owned `Send` capture may be retained by the compiler-managed
frame until completion or cancellation. `Share` does not grant mutable access.

## Diagnostics

The boundary analysis reports missing capability, borrowed capture across a
boundary, and mutable shared capture with the capture span primary and the
boundary span secondary. It does not silently downgrade a transfer to a
borrow or infer a detached lifetime.

## Consequences

Thread and coroutine metadata can validate closure captures without exposing
Rust-style reference syntax or changing the ordinary value ABI. HIR/MIR
consumers must retain capture mode, capability, scope, and cleanup facts when
closure construction and invocation are lowered. Closure transfer lowering
remains compiler-private and host-linking-specific.

## Dependencies

ADR-0035, ADR-0037, ADR-0088.

## Deferred

Coroutine suspension of closure bodies, detached execution, shared mutable
state, synchronization, channels, reflection, dynamic loading, public closure
layout, and FFI remain deferred.
