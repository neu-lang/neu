# ADR-0116: Non-panicking `Option` and `Result` combinators

Status: Accepted

## Decision

The initial pure-Neu standard library exposes only non-panicking combinators
whose operands and result retain the same generic parameters:

- `Option<T>.and(other: Option<T>): Option<T>`
- `Option<T>.or(other: Option<T>): Option<T>`
- `Result<T, E>.and(other: Result<T, E>): Result<T, E>`
- `Result<T, E>.or(other: Result<T, E>): Result<T, E>`

`and` returns the receiver when it is the failure/empty variant and otherwise
returns `other`; `or` returns the receiver when it is the success/present
variant and otherwise returns `other`. These operations do not panic and do
not depend on `stdlib`, I/O, closures, references, mutation, or error
conversion.

Each operation is tested by a native `public test func` in the same source file
as its implementation. Mapping, filtering, flattening, transposition,
`unwrap`-style operations, and type-changing combinators remain deferred until
the language has the required closure, reference, and generic substitution
semantics.
