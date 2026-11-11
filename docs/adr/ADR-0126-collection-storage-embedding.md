# ADR-0126: Collection Storage Embedding

Status: Accepted; revises the embedding deferral in ADR-0118

## Question

ADR-0118 defers collection-level embedding of `Array<T>`, so `Vector<T>` must
currently be an enum whose variants are public. Should a generic class be able
to own an `Array<T>` as a private field, so that a collection hides its
representation behind methods?

## Decision

- A generic class may declare a `private` field whose type is `Array<T>`, where
  `T` is the class's own type parameter. The field is owned by the instance and
  is accessible only inside the declaring class (ADR-0096).
- A collection's public surface is its methods. Its storage is not a public
  variant, field, or constructor argument.
- Construction uses explicit type arguments, `new Vector<T>(...)`, as ADR-0083
  requires. Its arguments are moved into the instance. Destruction of the
  instance destroys the array (ADR-0067).
- Inside the declaring class, methods may call operations on the array field.
  Outside it, callers reach the storage only through the class's methods.
- Checked access returns `core.Option<T>` and does not panic on an out-of-range
  index, as ADR-0111 requires for recoverable failures.
- Send and Share follow the element type. A collection is Send or Share exactly
  when its element type is, because the array field carries that capability
  (ADR-0014, ADR-0089).

## Scope

This ADR authorizes embedding for `Vector<T>` only. It does not add:

- `Slice<T>` or any other borrowed view. Those remain under ADR-0118 until the
  borrow analyses are wired into the driver (#51, #64).
- `Deque<T>`. It is removed from the package until its contract is accepted.
- Capacity policy, hashing, ordering, or iterator adapters.
- Implicit copying. A collection is copied only by an explicit clone, which
  waits for the `Clone` protocol (#62).

## Required compiler support

The following are not implemented, and the implementation must add them with
tests written first:

1. Explicit type arguments in `new C<T>(...)`. Today this is a
   `malformed_call_expression`.
2. Method calls on an instance of a generic class. Today `new Box(7)` assigned
   to `Box<Int>` with a method returning `T` is a `return_type_mismatch`.
3. Method calls on an `Array` field from inside its class. Today
   `items.size()` in a class method is an `unsupported_type_rule`.

## Unresolved questions

The project owner approved the decision above. These questions are not decided
by this ADR and need a separate decision before the behavior they affect is
implemented.

1. Should a `Vector<T>` whose element type is move-only be move-only, with no
   implicit copy? Undecided.
2. Should embedding be limited to `Array<T>`, or should a private field of any
   stdlib collection type be allowed later? This ADR limits it to `Array<T>`.

## Consequences

If accepted, `stdlib/collections/vector.neu` becomes a single generic class with
private storage. The enum variants are removed, and `Deque` is removed with
them. The compiler gains the three capabilities above, and ADR-0118's
deferral is limited to slices and the other items in its Scope section.
