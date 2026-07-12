# ADR-0070: Final-Only Runtime Dispatch

Status: Accepted

## Question

How should Neu express overridability and implement compiler-private runtime
dispatch for class and interface calls without exposing object or table ABI?

## Competing Designs

1. Classes and methods are overridable by default; `final` opts out, `override`
   is required for replacements, and runtime class/interface dispatch uses
   compiler-private slots.
2. Preserve `open` as an opt-in overridability modifier and make all other
   methods direct or final.
3. Make every call statically resolved from the declared receiver type.
4. Expose stable public vtable and interface-table layouts.

The second design conflicts with the accepted Kotlin-like default hierarchy
direction requested for Neu. Static-only resolution is unsound for virtual
overrides and interface values. Public layouts would constrain host linking,
separate compilation, and future ownership representation. The first design
preserves source ergonomics while keeping ABI details private.

## Decision

Classes and methods are overridable by default. `final class Name` cannot be a
superclass. `final fun name(...)` cannot be overridden. `override fun name(...)`
is required whenever a class method replaces an inherited class or interface
method. `open` is not an accepted declaration modifier; its lexer token remains
reserved only so declaration-position uses receive a source diagnostic. No
compatibility alias or silent migration is provided.

Interface declarations and required interface methods cannot be `final` or
`override`; they define required slots. Constructors are neither `final` nor
`override` and are not dispatch targets. Private methods are non-virtual and
cannot be override targets. Public and internal class methods are virtual by
default unless `final`; a method that has no inherited replacement and is not
private may still be called directly when the compiler proves its nominal target
is final or otherwise non-overridable.

An override must match the inherited method's parameter types, return type,
visibility domain, receiver ownership effect, inferred parameter effects,
thread-safety capabilities, and interface/class method identity. Visibility may
not be weakened or widened in a way that violates the inherited declaration.
An `override` without a compatible inherited target, an override of a final
method, and a subclass of a final class are diagnostics with declaration spans.
Missing `override`, incompatible interface implementations, and ambiguous
interface requirements are diagnostics before HIR lowering.

Inherited lookup walks the immediate superclass chain and the nominal interface
identity graph. A class method identity includes its declaring class and method
declaration. An interface implementation identity includes the declaring
interface, required method declaration, and implementing class. One compatible
class method may satisfy multiple identical interface requirements; conflicting
signatures or incompatible ownership/capability contracts are ambiguous
interface diagnostics.

`this.method(...)` and `receiver.method(...)` use the receiver's runtime class
dispatch contract. A base-typed receiver dynamically selects the most-derived
compatible override. An interface-typed receiver dynamically selects the
implementing class's interface slot. `super.method(...)` always selects the
immediate superclass method statically and never performs virtual or interface
dispatch. Receiver evaluation precedes explicit arguments, and arguments remain
left-to-right. `this` is the implicit non-consuming receiver under ADR-0065 and
ADR-0062.

The frontend records one of direct, virtual-class, interface, or static-super
dispatch facts. HIR and MIR preserve the nominal receiver type, method identity,
override target, dispatch kind, slot identity, ownership/effect facts, source
span, return type, and argument order. Direct dispatch is valid for private,
final, and compiler-proven non-overridable methods. Virtual dispatch loads the
compiler-private class slot. Interface dispatch loads the compiler-private
interface slot. Static-super dispatch names the immediate superclass method.

Vtables and interface tables are compiler-generated, object-owned metadata with
target-specific private layouts. Their lifetime is tied to the object and its
compiler-managed cleanup. No table pointer, slot number, object layout, symbol
name, or dispatch ABI is stable across modules, compiler versions, targets, or
FFI boundaries. Separate compilation exports nominal method identities,
signatures, override relationships, capabilities, ownership-effect metadata,
and dispatch slot identities; it does not export raw table layouts. Missing,
stale, or incompatible metadata is a diagnostic rather than a linker guess.

Nullable receivers must be flow-refined by the existing accepted rules before
dispatch. No safe-call, force-unwrap, cast, downcast, or runtime type-test
syntax is added by this ADR. A dispatch failure or invalid non-null receiver
state traps according to the existing host-linking trap contract; ordinary valid
dispatch does not require a standard library or scheduler runtime.

Host linking needs no public dispatch API. They must support the existing
compiler-private pointer/value ABI, executable object emission, and trap path.
The compiler rejects a host runtime that cannot represent the selected internal
dispatch operations before linking.

Multiple class inheritance, default interface methods, reflection, dynamic
loading, multiple dispatch, runtime type tests and downcasts, nullable dispatch
refinement beyond existing flow typing, FFI vtables, public object/table ABI,
and new reference, move, or lifetime syntax remain deferred.

## Supersession

This ADR supersedes the `open`-gating and default-final portions of ADR-0065
and ADR-0066. It preserves their nominal identity, single-superclass,
interface, visibility, ownership, `this`, `super`, and public-ABI constraints.
It also supersedes the conflicting `open` wording in the corresponding
`docs/SPEC.md` sections.

## Dependencies

- ADR-0010
- ADR-0017
- ADR-0018
- ADR-0025
- ADR-0026
- ADR-0046
- ADR-0047
- ADR-0062
- ADR-0065
- ADR-0066
- ADR-0067
- ADR-0069
