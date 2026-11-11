# ADR-0134: Generic Interface Default Methods

Status: Accepted; extends ADR-0133, which deferred default bodies in generic
interfaces

## Question

ADR-0133 accepts default methods for non-generic interfaces and rejects a
default body in a generic interface. A collection interface such as `List<T>`
needs default methods that call its own type parameter, for example
`func first(): T { return this.get(0); }`. How should such a default work with
the erased representation of generic type parameters (ADR-0126)?

## Decision

- A default body in a generic interface `interface I<T>` is accepted. Its body
  is checked once, with `T` as the interface's type parameter.
- Inheritance, replacement, and ambiguity follow ADR-0133. A class method
  matches an interface method when the names match and the parameter types match
  after the class's interface type arguments are substituted for the interface's
  type parameters. For example, `class IntBox: Box<Int>` with
  `func put(value: Int): Int` satisfies `put(value: T)` of `Box<T>`, so a default
  such as `putTwice(value: T)` that calls `this.put(value)` dispatches to it.
- Calls from a default body to an interface method, such as `this.get()`,
  dispatch through the interface as ADR-0133 requires. The call's result and
  arguments have the erased type of the interface's type parameter.

### Representation

- An erased slot of a generic interface method is a 64-bit word (ADR-0126).
  Every implementation of that method uses the same word at that slot, so that
  one interface call site can invoke any implementation.
- `Int` and reference types are already one word, so their implementations keep
  their native representation.
- `Bool`, `Byte`, and `Float` are narrower than a word. An implementing method
  whose own type at an erased slot is one of these is compiled with a word at
  that slot. The compiler encodes its return value and decodes its parameters at
  entry, as it does for generic methods under ADR-0126.
- A call to such a method, direct or through the interface, encodes arguments
  and decodes results by the method's effective signature, exactly as a call to
  a generic method does under ADR-0126.

## Scope

This ADR does not add:

- `super` calls to a default method. They remain a deferred diagnostic.
- Generic methods declared on an interface, or interface-level type parameters
  beyond those of the interface itself.

## Consequences

ADR-0133's deferred diagnostic for generic interface defaults is removed. The
compiler gains an effective-signature rule for implementations of generic
interface methods, and parameter decode and return encode wrappers for them.
Tests cover `Int`, `Bool`, and `Float` implementations, dispatch through a
generic interface type, and class and superclass replacement of a generic
default.
