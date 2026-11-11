# ADR-0133: Default Interface Methods

Status: Accepted; extends ADR-0065, which deferred default method bodies

## Question

ADR-0065 declares interfaces with required methods only, and defers default
bodies. Collections need shared behaviour, such as `contains` written in terms
of `get`, that every implementing class receives without copying it. Should an
interface method with a body supply a default implementation?

## Decision

### Declaration

- An interface method declared with a body is a default method. One declared
  with `;` is required, as before.
- An interface has no state, so a default body uses only its parameters, its
  locals, `this`, and the interface's methods. It may not name a field.
- A default method is not `final`. `final` on an interface method remains a
  diagnostic.

### Inheritance

A class that implements an interface inherits each default method, unless one
of the following supplies the method:

1. The class declares a method with the same name and parameter types. That
   replacement is an override under ADR-0127 and must have a compatible return
   type.
2. A superclass declares a concrete method with the same name and parameter
   types. The superclass method wins over the interface default.

### Dispatch

- `this` in a default body has the interface type. A call on `this` is an
  interface call, as a call on any interface-typed receiver: it selects the
  receiving object's most-derived implementation, which is the class's own
  method, an inherited superclass method, or the default.
- A call on an interface-typed receiver selects the same implementation.

### Conflicts

- A class that inherits the same name and parameter types from two interfaces,
  and declares no method of its own, is `AmbiguousInterfaceMethod`. This holds
  whether one or both are defaults, and whether they are required or default.
- A class must still implement every required method, as before.

### Deferred

- Default bodies of generic interfaces, such as `List<T>`, are deferred. The
  compiler reports them as unsupported until the substitution of type arguments
  into default bodies is decided.
- `super` calls to an interface default (`Iface.super.m()`).
- Interface state and fields.

## Consequences

- An interface can share behaviour across its implementations.
- ADR-0065's statement that default bodies are deferred is superseded by this
  ADR.
