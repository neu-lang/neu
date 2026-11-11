# ADR-0129: Dynamic Array Type Syntax

Status: Accepted; replaces the `Array<T>` spelling in ADR-0073 and ADR-0119

## Question

ADR-0073 writes the dynamic array type as `Array<T>`, and the compiler treats
`Array` as a built-in type name. Should the dynamic array type keep that name
and spelling?

## Decision

The dynamic array type is written `T[]`, with an empty bracket pair, as
`Int[]`, `Box<Int[]>`, or a generic element `T[]` in a generic declaration.

- `Array` is not a built-in type name, and `Array<T>` is an unresolved type.
- `new T[]` still creates an empty dynamic array, as ADR-0073 specifies.
- Fixed arrays `T[N]` (ADR-0063) are unchanged.
- The element rules of ADR-0073 and ADR-0119 are unchanged: bootstrap scalars,
  and generic type parameters. Nested dynamic arrays, `T[][]`, remain deferred.
- Operations, mutation, move-only ownership, and traps are unchanged.

## Consequences

- The name `Array` is free for user declarations.
- Source, tests, and documentation use `T[]`. Historical ADR text keeps the old
  spelling, and this ADR supersedes it.
