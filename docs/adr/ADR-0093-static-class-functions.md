# ADR-0093: Static Class Functions

## Status

Accepted.

## Decision

Class bodies may declare associated functions with `static func name(...)`.
The modifier is accepted only on class functions, before `func` and after any
visibility or `final` modifier. A static class function has no implicit
receiver and is called through a nominal class name, for example
`Math.answer()`. Calling it through an instance is rejected. Interfaces do
not declare static functions.

Static functions cannot read `this`, `super`, instance fields, or instance
methods. Their ownership effects, capabilities, generic substitution,
visibility, parameter evaluation, return behavior, and diagnostics are the
same as for top-level functions, with no receiver parameter. Constructors are
not static functions.

Static functions participate in same-module overload resolution by owner,
name, static kind, and ordered parameter types. A derived class inherits an
unhidden static function for class-name lookup; a same-named declaration in the
derived class hides the inherited declaration and is not an override target.
`override` on a static function is rejected. `final` is accepted as redundant
metadata and does not change hiding behavior. Private visibility remains
enforced by the existing module and owner rules.

Class-name lookup uses nominal class identity, not text-only matching, and
preserves the selected declaration across separate compilation metadata. HIR
records a static direct-call fact with the class and function identity; MIR,
Cranelift, object emission, and linking lower it as a compiler-private direct
call with the ordinary value ABI. No virtual or interface receiver is passed,
and no public class-object, symbol, layout, or FFI ABI is introduced.

## Consequences

The parser must preserve static class-function metadata and reject static
functions in interfaces. Type checking must distinguish class-name receivers
from instance receivers, enforce receiver-free bodies, inherited hiding, and
visibility. Existing enum-associated functions remain governed by ADR-0081.

## Dependencies

ADR-0068, ADR-0070, ADR-0075, ADR-0076, ADR-0081, ADR-0082, ADR-0084,
ADR-0085.

## Deferred

Static fields, class objects, reflection, dynamic lookup, multiple class
inheritance, default interface static methods, public or stable symbol/layout
ABIs, FFI, and runtime class metadata APIs remain deferred.
