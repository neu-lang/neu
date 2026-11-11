# ADR-0132: Dynamic Array Type Syntax

Status: Accepted; supersedes ADR-0131 (not merged), and replaces the `Array<T>`
spelling in ADR-0073 and ADR-0119

## Question

Library containers such as `ArrayList<T>` need storage whose length is chosen
at run time. ADR-0073 provides that as a built-in type spelled `Array<T>`. The
project owner removes the `Array` spelling and does not want a compiler-provided
`Buffer<T>`. What spelling should the built-in dynamic array have?

## Decision

The built-in dynamic array type is written `T[]`, with an empty bracket pair:
`Int[]`, `Bool[]`, a generic element `T[]` in a generic declaration, and
`Box<Int[]>` as a type argument (ADR-0128).

- `Array` is not a type name. `Array<T>` is an unresolved type.
- `new T[]` creates an empty dynamic array, as in ADR-0073.
- The operations of ADR-0073 are unchanged: `add`, indexed `add`, `remove`,
  `size`, element reads and writes, for-each, and move-only ownership. Out-of-
  bounds reads and writes trap.
- The element rules of ADR-0073 and ADR-0119 are unchanged: the bootstrap scalars
  `Bool`, `Int`, `Float`, and `Byte`, and generic type parameters. Nested dynamic
  arrays, `T[][]`, remain deferred.
- Fixed arrays `T[N]` (ADR-0063) are unchanged. A bracket pair with a length is
  fixed; an empty bracket pair is dynamic.

## Consequences

- `ArrayList<T>` is ordinary Neu code over `T[]`, with private fields
  (ADR-0130).
- The name `Array` is free for user declarations.
- Source, tests, examples, and documentation use `T[]`.
