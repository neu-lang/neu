# ADR-0082: Generic Type Identity And Substitution

## Status

Accepted.

## Decision

Generic type arguments are explicit, type-only arguments written in angle
brackets, such as `Box<Int>` and `Map<String, Array<Byte>>`. Generic argument
lists have exact arity. Parameters are invariant: there are no variance
annotations, wildcards, star projections, implicit conversions, or inferred
arguments.

A generic parameter is identified by its declaring generic declaration and
parameter declaration node. Its source name is only a lookup spelling and does
not determine identity. Duplicate parameter names in one declaration and
unresolved parameters are diagnostics; nested declarations may shadow outer
parameter names without changing either declaration's identity.

A generic instance is identified by the declaring nominal type identity and the
ordered substituted argument `TypeId`s. `Box<Int>` and `Box<Byte>` are distinct,
and repeated construction of the same identity returns the same type-arena
record. Nested generic arguments, nullable arguments, fixed arrays, dynamic
arrays, and accepted nominal arguments retain their structural identities.

Substitution is an explicit compiler operation over a parameter-to-type
environment. It recursively substitutes generic parameters through nullable,
fixed-array, dynamic-array, and generic-instance types. It does not infer
arguments, solve constraints, erase types, monomorphize code, or define runtime
layout. Missing substitutions remain generic parameters and are diagnosed only
when a later declaration or use requires a concrete type.

Generic parameters inherit ownership, capability, and nullability behavior
only from their accepted bounds and substitutions. This ADR does not add
variance, aliases, recursive generic declarations, higher-kinded types,
dependent types, function-type arguments, or cross-module generic lookup.

Generic identity metadata is compiler-private and source-mapped. It is not a
public ABI or FFI contract.

## Consequences

The type arena gains deterministic structural generic-instance records and an
explicit substitution API. Generic declarations, constraint solving,
specialization, and runtime lowering remain later decisions.

## Dependencies

ADR-0016, ADR-0023, ADR-0032, ADR-0035, ADR-0063, ADR-0065, ADR-0075,
ADR-0076.
