# ADR-0094: Abstract Classes And Functions

## Status

Accepted.

## Decision

`abstract class Name(...)` declares a non-instantiable class. It may contain
concrete fields, the existing primary constructor, concrete methods, and
abstract instance functions. `abstract func name(...): T;` is declaration-only:
it has no body, no `static` modifier, and uses the ordinary instance receiver,
ownership, capability, overload, and signature rules. Abstract classes may
inherit from classes and implement interfaces.

An abstract class may remain abstract through any number of subclasses. A
non-abstract class is constructible only when every inherited abstract function
identity has a compatible concrete implementation in its hierarchy. The
implementation must use `override`; incompatible signatures, missing
completion, invalid visibility, abstract bodies, `abstract final`, and
`abstract static` combinations are diagnostics. Completing an abstract
function preserves its nominal method identity and existing virtual or
interface dispatch facts.

Abstract functions cannot be invoked as unresolved implementations, and
constructing an abstract class is diagnosed before HIR lowering. Nullable
receivers still require the existing refinement. Abstract classes may contain
static functions under ADR-0093, but abstract static functions are not allowed.

HIR and MIR retain abstract method identity, completion targets, receiver
effects, source spans, and dispatch metadata for validation; only concrete
implementations are emitted. The private value/object ABI, target packs,
object format, linking, and runtime ownership model are unchanged. No public
abstract-object, vtable, reflection, dynamic-loading, or FFI ABI is defined.

## Dependencies

ADR-0065, ADR-0068, ADR-0069, ADR-0070, ADR-0075, ADR-0076, ADR-0093.

## Deferred

Traits, mixins, multiple inheritance, default interface methods, reflection,
dynamic loading, public layout or dispatch ABI, FFI, and runtime type tests
remain deferred.
