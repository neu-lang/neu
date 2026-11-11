# ADR-0130: Private Primary-Constructor Parameters

Status: Accepted; revises ADR-0069, which made promoted parameters public

## Question

ADR-0069 makes a promoted primary-constructor parameter a public field, and
ADR-0067 requires every field to be initialized by the primary constructor.
A class whose storage must stay private, such as a `Vector<T>` over an
array, cannot declare that storage. Should a promoted parameter be allowed to
be private?

## Decision

A promoted primary-constructor parameter may carry the `private` modifier:

    class Holder(private var value: Int) { ... }

- A private promoted field is readable and writable only inside its class,
  as any private field is. Access from outside the class is a diagnostic.
- Promoted parameters without `private` remain public, as ADR-0069 specifies.
- Initialization is unchanged: the constructor initializes the field exactly
  once, as ADR-0067 specifies.
- `protected` remains deferred.

## Consequences

- A class can hold private storage that is initialized by its constructor.
- ADR-0069's rule that promoted parameters are public no longer holds for
  parameters that write `private`.
