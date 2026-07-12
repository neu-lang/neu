# ADR-0085: Generic Specialization And Private ABI

## Status

Accepted.

## Decision

Concrete generic calls use explicit type arguments immediately before the call
argument list, for example `identity<Int>(1)`. A specialization is identified
by the generic declaration node and ordered concrete `TypeId` arguments. The
compiler deduplicates equal identities and rejects recursive specialization
requests before backend lowering.

Specialization occurs after type checking and constraint validation and before
MIR lowering. Substituted HIR/MIR facts include ownership, cleanup, aggregate,
dispatch, and source mappings. Compiler-private symbols encode the base
function identity plus the ordered specialization type identities; they are
not source-visible, stable across compiler versions, or usable through FFI.

The initial backend accepts only type arguments with an already accepted
bootstrap ABI representation. Unsupported aggregate or target-pack
capabilities are diagnostics before object emission. Specializations are
deduplicated within a compilation and recursive expansion is rejected. No
runtime erasure, reflection, public layout, separate-compilation cache, or
type inference is introduced.

## Consequences

Call metadata must preserve explicit generic arguments. HIR/MIR can associate a
call with a compiler-private specialization identity, while later work may
instantiate executable bodies using the established substitution API.

## Dependencies

ADR-0075, ADR-0082, ADR-0083, ADR-0084.
