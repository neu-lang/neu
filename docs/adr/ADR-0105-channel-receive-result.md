# ADR-0105: Channel Receive Result

## Status

Accepted.

## Question

What typed Neu value does `receive(channel)` return so that a received message
can be distinguished from end-of-stream, including when the channel element
type itself can represent an empty or nullable value?

## Alternatives

1. Return a nullable `T` and reserve null for end-of-stream.
2. Return a compiler-provided result value with separate message and closed
   variants.
3. Return a tuple containing a status and an optional payload.

## Trade-offs

Nullable return values make a valid nullable message indistinguishable from
closure. Tuples expose a convention that every caller must interpret and do
not provide a nominal identity for the state transition. A result value keeps
the state and payload distinct, uses the accepted payload-enum matching model,
and preserves ownership of the received value without adding reference or
error syntax.

## Decision

`receive(channel)` returns the compiler-provided nominal type
`ChannelResult<T>`. It has exactly two variants:

- `ChannelResult.Message(value)`, carrying one value of the channel element
  type `T`;
- `ChannelResult.Closed`, carrying no payload.

`ChannelResult<T>` is available only as the result of the compiler-recognized
channel operation. Users do not declare, extend, or construct this type through
an allocation API. Existing payload-bearing `when` patterns from ADR-0081 are
used to inspect it. The subject is evaluated once and ordinary exhaustiveness
and arm typing rules apply.

The result is move-only when `T` is move-only and copyable when `T` is
copyable, following the existing aggregate ownership rules. Matching a
`Message` transfers or copies its payload according to those rules. A
`Closed` result has no payload. A message is therefore never represented by a
sentinel value, including when `T` is nullable or otherwise has an empty value.

This ADR defines the receive result only. Capacity, FIFO ordering, blocking,
closure, cancellation, receiver multiplicity, cleanup, and channel capability
rules remain those of ADR-0103. `send` and `close` retain their existing
compiler-recognized operations and failure boundaries.

The result type and its representation are compiler-private. HIR and MIR
preserve the result type, variant identity, payload ownership, suspension,
closure state, and source spans. No public layout, stable ABI, FFI contract,
general result type, or standard-library module is introduced.

## Deferred

Select operations, pattern guards, user-defined channel result types,
unbounded channels, channel iteration syntax, external I/O, and any public
option/result library remain deferred.

## Consequences

- ADR-0103's explicit end-of-stream state has a precise typed contract.
- `T?` is not used as a channel EOF convention.
- Channel implementation tasks may proceed in their existing order, beginning
  with the contract task, without inventing a receive result type.

## Dependencies

- ADR-0081
- ADR-0035
- ADR-0062
- ADR-0101
- ADR-0103

## Supersession

This ADR supersedes only the unresolved receive-result-type and receive-source
construction portions of ADR-0103. All other ADR-0103 decisions remain in
force.
