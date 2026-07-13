# ADR-0113: Collection Protocol Contracts

Status: Accepted

## Question

Which library protocols may collection algorithms require before associative
collections and generic algorithms are implemented?

## Decision

Collection protocols are ordinary library interfaces, not compiler-recognized
capabilities. A type may be used with an operation only when the operation's
signature explicitly requires the protocol and the implementation can dispatch
that protocol through the accepted nominal/interface rules.

`Eq<T>` compares two values of the same nominal type and is reflexive for
values that can be stored in a set or map. `Ord<T>` provides a deterministic
total ordering for ordered collections; it does not silently fall back to
pointer identity or source order. `Hash<T>` produces a deterministic hash for
the value and is valid only with an equality relation whose equal values have
equal hashes. `Clone<T>` creates an independent owned value, while `Default<T>`
constructs the documented empty/default value. None of these protocols imply
`Copy`, `Send`, or `Share`.

`HashMap<K, V>` and `HashSet<T>` require `Eq` and `Hash` for their keys or
elements. `BTreeMap<K, V>` and `BTreeSet<T>` require `Ord`. Sorting, binary
search, deduplication, and lexicographic comparison state their exact
protocol requirements and return an explicit result when a documented
precondition can fail. A collection never invents equality, ordering, or
hashing from a representation detail.

Protocol method resolution is nominal and explicit in the first phase. There
is no structural duck typing, blanket implementation, reflection, implicit
conversion, or compiler privilege for the standard library. Protocol dispatch
and separate compilation remain bounded by the accepted generic and ABI
contracts.

## Non-goals

This ADR does not define protocol syntax, specialization priority, blanket
implementations, hashing randomization, serialization, formatting, or
allocation failure. It does not make `Eq`, `Ord`, `Hash`, `Clone`, or `Default`
compiler-recognized capabilities.

## Consequences

Associative collections and algorithms have checkable prerequisites and cannot
silently depend on unstable representation behavior. Core can implement the
protocol declarations independently of collection storage, while collection
implementations remain pure Neu source.

## Required follow-up

Accept this ADR (or a superseding decision), add protocol signatures and
dispatch diagnostics to `docs/SPEC.md`, and add negative tests for missing
protocols and inconsistent map/set key requirements before implementing
associative collections.

## Dependencies

This proposal depends on ADR-0107, ADR-0108, ADR-0109, ADR-0110, ADR-0014,
ADR-0016, and ADR-0076. It is based on the #19 contract branch so related
issue work remains linear.
