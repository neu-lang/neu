# ADR-0065: Class And Interface Foundation

Status: Accepted

## Question

What foundational nominal, member, visibility, receiver, nullability, and
ownership rules define classes and interfaces before their implementation?

## Competing Designs

1. Kotlin-like nominal classes with one superclass, interfaces, fields, and
   compiler-inferred receivers.
2. Struct-like value types with structural interfaces and no object identity.
3. Multiple class inheritance with unrestricted member reuse.
4. Defer all nominal runtime types until a standard library exists.

The struct-like design conflicts with the language's nominal type decision.
Multiple class inheritance creates ambiguous storage and dispatch rules.
Deferring the model would leave parser and type-checker work unable to state
what a declaration means. The Kotlin-like model provides familiar syntax while
keeping inheritance and runtime layout separately constrained.

## Decision

Classes and interfaces have nominal identity consisting of the declaring
module, package, declaration identity, and generic arguments when generics are
later accepted. A class has at most one direct superclass and may implement
multiple interfaces. An interface may extend multiple interfaces. Structural
class conformance, traits, mixins, nested classes, companion objects, and
multiple class inheritance are deferred.

Class declarations may contain fields and methods. Fields use `val` or `var`
and require declared types and initializers under ADR-0067. Methods use `fun`.
The default declaration visibility is `internal`; `public`, `internal`, and
`private` follow ADR-0025. `protected` is not a declaration visibility in the
foundation and is deferred rather than silently treated as `internal`.

An interface declares required methods. Default method bodies, interface state,
extension methods, and operator overloads are deferred. An implementing class
must provide each required method with a compatible signature and explicit
`override`; missing, duplicate, or ambiguous implementations are diagnostics.

`this` is an implicit non-null receiver in instance methods and may be written
explicitly as `this.` when shadowing requires qualification. Receiver lookup
searches the current class, then inherited members under ADR-0066; local names
shadow members. `this` is not available before construction is complete, cannot
be rebound, and cannot escape through a field or returned value during
construction.

Class and interface values are nullable only when written with the accepted
postfix nullable type form. A non-null receiver is required for member access.
Null checks may refine a nullable receiver only through the existing flow
typing rules. Safe access, casts, type tests, and downcasts are deferred until
their own expression and runtime contracts are accepted; invalid nullable
access is therefore not given a new syntax here.

Instances and fields are move-only by default. A field owns its stored value;
no implicit copy or aliasing is created. Receiver reads use an implicit shared
borrow, receiver mutation requires an implicit exclusive effect, and no method
implicitly consumes `this`. Classes are `Send` only when all owned fields and
the superclass satisfy the existing capability rules. `Share` requires an
immutable class state and transitively shareable owned fields. Cyclic ownership
graphs are rejected or deferred; this ADR does not add a tracing collector or
reference syntax.

## Diagnostics

Implementations must provide source-mapped diagnostics for duplicate nominal
declarations, unsupported inheritance, missing interface methods, invalid
override declarations, inaccessible members, invalid receiver use, use of
`this` before construction, nullable receiver access, and ownership or
capability violations. Diagnostics must identify the declaration or access
span and the relevant nominal identity.

## Consequences

Later implementation may parse and type nominal declarations without deciding
object layout. Dispatch rules come from ADR-0066 and construction, storage,
and destruction rules come from ADR-0067. Generic classes and interfaces,
protected members, defaults, casts, runtime type tests, reflection,
serialization, and FFI remain deferred.

## Dependencies

- ADR-0010
- ADR-0013
- ADR-0014
- ADR-0017
- ADR-0025
- ADR-0026
- ADR-0027
- ADR-0035
- ADR-0036
- ADR-0037
- ADR-0062
- ADR-0066
- ADR-0067
