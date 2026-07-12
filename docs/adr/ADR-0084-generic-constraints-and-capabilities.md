# ADR-0084: Generic Constraints And Capabilities

## Status

Accepted.

## Decision

Generic bounds are explicit conjunctions in declaration order. A type argument
must satisfy every bound after substitution, before HIR lowering or backend
selection. Bounds are invariant and never create conversions or inferred type
arguments. `Send` and `Share` use the existing thread-capability predicates;
ownership classification and nullable propagation use the existing ownership
and type rules.

Capability names are validated deterministically. A substituted concrete type
that fails `Send` or `Share` produces a source-mapped constraint diagnostic.
An uninstantiated generic parameter preserves its bound proof obligation and
is not treated as satisfying or violating the bound. Class and interface bound
names retain nominal identity and are checked against the enclosing module's
accepted class/interface records when an instantiation context is available.
Unknown bounds, unresolved substitutions, recursive bound cycles, and stale
separate-compilation proofs are diagnostics; no fallback or first-match choice
is allowed.

Bounds are checked in source order and all failures are retained. Substitution
is applied recursively through accepted nullable, fixed-array, dynamic-array,
and generic-instance constructors. Constraints do not affect overload
identity; they only admit or reject a candidate after exact invariant identity
is established. No inference, variance, wildcards, higher-kinded constraints,
runtime reflection, or public ABI is introduced.

## Consequences

The compiler exposes constraint proof records and deterministic capability
diagnostics for later specialization. Generic values remain conservative for
ownership and thread safety until a concrete proof is available.

## Dependencies

ADR-0032, ADR-0035, ADR-0037, ADR-0063, ADR-0064, ADR-0082, ADR-0083.
