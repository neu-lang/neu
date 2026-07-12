# ADR-0103: Bounded Channel Contract

Status: Accepted; receive result completed by ADR-0105

## Decision

Neu provides compiler-recognized bounded channels through
`channel<T>(capacity)`, `send(channel, value)`, `receive(channel)`, and
`close(channel)`. `capacity` is a non-negative `Int` compile-time or runtime
value. Capacity zero is a rendezvous channel; positive capacity permits that
many queued messages. Unbounded channels are not supported.

Channels are opaque compiler-owned values. A channel may have multiple senders
and one receiver. Sender and receiver operations use compiler-managed shared
channel state; no public reference or synchronization syntax is introduced.
The element type is exact and every sent value must satisfy the existing `Send`
capability. Move-only values are consumed when a send completes. Copyable
values remain usable under normal copy rules.

Messages are FIFO. `send` suspends when the channel is full and resumes when
space is available. A send on a closed channel completes with the defined
channel-closed failure and does not silently drop the value. `receive` suspends
when empty, returns the next message as `ChannelResult.Message(value)` when
available, and returns `ChannelResult.Closed` after closure and draining. The
typed result contract is defined by ADR-0105. Closing is idempotent; no new
send succeeds after closure. A blocked send or receive observes structured
cancellation and releases its pending operation before the task completes.

The receiver may be used by only one logical consumer at a time. Concurrent
receiver use is a deterministic diagnostic rather than an implicit work-sharing
policy. Channel cleanup closes the channel and releases queued messages in FIFO
order at the owning scope boundary. No network, process, file, select, timer,
or unbounded-buffer behavior is included.

HIR and MIR preserve channel identity, element type, capacity, operation state,
message ownership, suspension points, closure, cancellation, cleanup, and
source spans. Channel layout and runtime symbols remain private with no stable
ABI or FFI contract.

## Dependencies

- ADR-0101
- ADR-0104
- ADR-0037
- ADR-0062
- ADR-0100
- ADR-0105
