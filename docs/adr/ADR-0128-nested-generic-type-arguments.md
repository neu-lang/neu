# ADR-0128: Nested Generic Type Arguments

Status: Accepted; extends the explicit type-argument rules in ADR-0083

## Question

ADR-0083 accepts explicit invariant type arguments for generic classes,
interfaces, and enums. It does not say which types may be arguments, and the
bootstrap accepts only identifier names. Should a type argument be any type,
including a generic instance or `Array<T>`?

## Decision

A type argument may be any type that a type annotation may denote:

- a primitive (`Int`, `Bool`, `Float`, `Byte`), a class or interface, a
  generic parameter in scope, or `Array<X>`;
- a generic instance `C<A, ...>`, to any depth, so `Box<Array<Int>>` and
  `Box<Box<Int>>` are valid.

The rule applies wherever a type argument appears: type annotations, `new`
expressions (`new Box<Array<Int>>(...)`), class and interface headers
(`class C: Base<Array<Int>>(...)`), and superclass arguments.

- Arguments are invariant (ADR-0083). `Box<Array<Int>>` is compatible only with
  `Box<Array<Int>>`, not with `Box<Int>` or `Box<Array<Bool>>`.
- Two spellings denote the same type when they resolve to the same type. Spaces
  are not significant.
- A type argument that does not resolve is reported as an unresolved type, as
  any other unresolved name is.

## Deferred

- Fixed arrays `T[N]` as type arguments.
- Function types as type arguments.
- Variance, type-argument inference, and default type arguments.

## Consequences

- A type argument is a type tree, not a list of names. The bootstrap resolves
  type arguments recursively, and compares them by resolved type.
- An annotation such as `Box<Array<Int>>` no longer resolves to a bare `Box`.
