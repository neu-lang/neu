# ADR-0106: Shared Channel Handles

## Status

Accepted.

## Question

How can multiple structured tasks use one opaque channel without exposing
references or requiring users to write ownership-transfer syntax?

## Decision

`Channel<T>` is a copyable compiler-owned handle to shared channel state. Copying
the handle copies access to the state; it does not copy queued messages. A
handle may be captured by multiple child tasks, passed to multiple senders, and
used by the one logical receiver permitted by ADR-0103. The compiler/runtime
tracks the private state lifetime and releases it after the owning structured
scope has completed and no handle remains in use.

Sending a message remains the ownership boundary: a move-only message is
consumed when `send` completes, while a copyable message remains usable. The
channel handle itself is not consumed by `send`, `receive`, or `close`.

Channel handles satisfy the accepted `Send` and `Share` capability checks. The
handle representation, reference accounting, queue storage, and cleanup
symbols remain compiler-private. No source-level reference, lifetime, atomic,
or allocator API is added.

## Consequences

- A channel can be captured by producer and consumer tasks without explicit
  borrow or move syntax.
- HIR and MIR preserve shared channel identity and capture facts separately
  from message ownership facts.
- Channel cleanup must not free shared state after the first handle disappears.
- Separate compilation exchanges only the `Channel<T>` semantic identity and
  capability contract, never its layout.

## Deferred

Multiple logical receivers, select, channel iteration, public channel types,
network channels, and user-defined synchronization remain governed by the
deferrals in ADR-0103.

## Dependencies

- ADR-0037
- ADR-0062
- ADR-0088
- ADR-0089
- ADR-0101
- ADR-0103
- ADR-0105
