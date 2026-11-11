# ADR-0127: Remove the `override` Keyword

Status: Accepted; removes the `override` keyword and supersedes its requirements
in ADR-0065 and ADR-0070, and the matching SPEC wording

## Question

ADR-0070 requires `override` on every replacement of an inherited method, and
ADR-0065 requires it for every interface implementation. Should the keyword
exist at all?

## Decision

`override` is removed. It is no longer a keyword, and no declaration may use it.

- A method replaces an inherited method when its name and parameter types match
  a superclass method or an interface requirement. No marker is written.
- A replacement must be compatible with the inherited method in parameter types,
  return type, visibility, receiver effects, and capabilities. An incompatible
  replacement is a diagnostic.
- A replacement of a `final` method is a diagnostic. A subclass of a `final`
  class is a diagnostic. `final` is unchanged.
- Missing interface implementations, ambiguous interface requirements, and
  abstract methods left unimplemented by a concrete class remain diagnostics.
- A static function with the same name hides the inherited static function and
  is not an override. That is unchanged.
- Writing `override` as a modifier is an unsupported-declaration-modifier
  diagnostic.

## Consequences

- A method added to a base class can silently replace a subclass method with
  the same signature. Java and C# prevent this with a marker. This decision
  accepts that risk.
- Existing programs that write `override` must remove it to compile.
- ADR-0065's and ADR-0070's requirement of an explicit `override` is superseded
  by this ADR.
