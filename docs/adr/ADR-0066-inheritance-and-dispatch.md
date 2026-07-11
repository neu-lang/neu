# ADR-0066: Inheritance And Dispatch

Status: Accepted

## Question

How do accepted classes override methods, implement interfaces, resolve
`this` and `super`, and dispatch across module boundaries?

## Competing Designs

1. One direct superclass, multiple interfaces, explicit overrides, and
   compiler-private direct, virtual, and interface dispatch.
2. Multiple class inheritance with linearized method resolution.
3. Single inheritance with all calls statically devirtualized.
4. Publicly stable vtables and object layouts for separate compilation.

Multiple class inheritance adds ambiguous fields and constructor order.
All-static dispatch cannot express virtual overrides. Public layouts constrain
future target packs and conflict with the opaque ABI direction. The first
design supports Kotlin-like ergonomics while keeping ABI details internal.

## Decision

A class has one direct superclass and zero or more interfaces. Interfaces may
inherit interfaces. A class or method is `final` unless the accepted class
declaration form explicitly marks it `open`; sealed class behavior is deferred.
Only an `open` method may be overridden. An override must be explicit and its
parameter types, return type, receiver ownership effect, visibility, and
capability obligations must be compatible with the overridden method.

Private methods are not virtual and are not override targets. Public and
internal methods may be virtual within their accepted visibility domain.
There is no protected visibility. Field hiding is rejected. A subclass accesses
inherited fields only through the inherited declaration's visibility rules.

`super.method(...)` is valid only in an instance method or constructor of a
derived class and selects the immediate superclass implementation. `super`
cannot be stored, returned, used as a first-class value, or used to bypass
visibility. Superclass construction is governed by ADR-0067. Unqualified
member lookup uses the current receiver; local bindings shadow members, and
`this.member` resolves the receiver member explicitly.

Dispatch is selected by the compiler. Non-overridden or final methods use
compiler-private direct dispatch. Overridable class methods use compiler-private
class dispatch metadata. Interface calls use compiler-private interface-table
metadata. No object, vtable, interface-table, symbol, or method layout is a
stable public ABI or FFI contract.

Interface implementation is checked by nominal identity and method signature.
A class must provide exactly one compatible implementation for each required
method. Conflicting inherited interface requirements are rejected unless one
explicit class method satisfies both with compatible contracts. Default
interface methods are deferred, so there is no default conflict resolution.

Separate compilation exports nominal method identities, visibility, method
signatures, override relationships, capability requirements, and inferred
ownership-effect metadata. It does not export a target-specific table layout.
Missing, stale, or incompatible metadata is a diagnostic rather than a linker
guess.

Downcasts, runtime type tests, nullable dispatch refinement, reflection,
multiple dispatch, dynamic loading, and FFI vtables are deferred. A nullable
receiver must be flow-refined before any dispatch; otherwise the source
diagnostic is emitted before HIR lowering. Receiver reads borrow implicitly,
receiver mutation requires exclusive access, and dispatch never adds a hidden
reference or consuming receiver parameter.

## Diagnostics

Required diagnostics include invalid superclass or interface lists, final or
missing overrides, incompatible override signatures, inaccessible `super`,
ambiguous interface implementations, field hiding, stale method metadata,
nullable receiver dispatch, and receiver ownership or capability violations.
Each diagnostic preserves declaration, call, or receiver source spans.

## Consequences

The compiler can define method identity and dispatch contracts before choosing
an object representation. Later lifecycle work must provide construction and
destruction around these dispatch rules. Public ABI and FFI remain unable to
depend on class or table layout.

## Dependencies

- ADR-0010
- ADR-0017
- ADR-0018
- ADR-0020
- ADR-0025
- ADR-0026
- ADR-0046
- ADR-0047
- ADR-0062
- ADR-0065
- ADR-0067
