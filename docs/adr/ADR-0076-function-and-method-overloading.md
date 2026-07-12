# ADR-0076: Function And Method Overloading

Status: Accepted by the main-task Chief Architect

## Question

How should Neu resolve declarations that share a function or method name while
remaining distinct from overriding and preserving compiler-private dispatch?

## Competing Designs

1. Support same-module top-level and method overload sets with exact parameter
   matching, then lower the selected declaration with its existing dispatch
   kind.
2. Select candidates using implicit numeric conversions, nullable widening, or
   declaration order.
3. Treat every repeated name as an error and require distinct source names.
4. Defer all overloads until generic and cross-module lookup exist.

Design 1 provides useful Kotlin-like ergonomics without making backend types or
declaration order part of source semantics. Design 2 makes ownership,
nullability, and ambiguity difficult to explain and stable across modules.
Design 3 rejects a useful language feature. Design 4 would block exact
overloads that do not depend on the deferred features.

## Decision

Top-level functions and class methods may form overload sets within one source
module and package. Interface declarations may contain overload sets. A
constructor has exactly the primary constructor form accepted by ADR-0069;
secondary and overloaded constructors remain deferred. Cross-module lookup and
overload resolution remain deferred under ADR-0026.

An overload identity is the declaring owner identity, function name, and
ordered parameter type identities. Return type, visibility, source order,
ownership effects, and capabilities are not part of identity. Duplicate
identities are diagnostics. Generic parameters, receiver extension types, and
function values do not participate because their overload forms are deferred.

Candidate collection uses the existing visibility and same-module lookup
rules. Top-level candidates use the unqualified function name. Method
candidates use the receiver's nominal static type, inherited lookup, and
interface identity. An inherited overload set is merged by identity. A
declaration with a matching inherited identity is an override candidate and
must use `override`; a declaration with a new identity is an overload and does
not need `override`. `final` and visibility rules from ADR-0070 still apply.
`super.method(...)` searches the immediate superclass overload set only.

Resolution first considers candidates whose ordered parameter types exactly
match the statically typed arguments, including nullable identity. If no exact
candidate exists, already accepted assignment compatibility may be considered
only when it yields exactly one candidate. Class-to-base and class-to-interface
compatibility therefore participates, but it has no ranking over another
compatible candidate. Numeric conversions, boxing, unboxing, default
arguments, named arguments, varargs, and implicit nullable construction are
not performed. Zero candidates produce a no-matching-overload diagnostic;
multiple candidates produce an ambiguous-overload diagnostic. Declaration order
never breaks a tie.

Argument evaluation remains left-to-right and candidate inspection never
consumes ownership. After one declaration is selected, its existing inferred
read, exclusive-mutate, consume, store, return-owned, receiver, and projection
effects are validated exactly once at the call site. An unresolved call never
reaches HIR, MIR, Cranelift, object emission, or linking.

The selected declaration identity is preserved separately from its overload-set
identity. Final and private methods lower as direct calls. Overridable class
methods lower as virtual calls, interface-typed calls lower as interface calls,
and `super` calls lower as static-super calls under ADR-0070. Overload
selection is compile-time; runtime dispatch selects only among implementations
of the already selected method identity and never performs runtime overload
resolution.

Compiler-private function symbols include the complete owner, name, and
ordered parameter type identity. Symbols, virtual slots, interface slots, and
overload metadata are not stable public or FFI ABI. Separate compilation must
provide the selected signature, ownership/effect contract, override identity,
and dispatch metadata; missing or incompatible metadata is a diagnostic.

Interfaces may declare multiple methods with one name when their parameter
identities differ. Implementations must provide one compatible `override` for
each required identity. Conflicting requirements with the same identity remain
the existing interface conflict diagnostic. Constructors, generic overloads,
operator overloads, extension methods, dynamic loading, reflection, multiple
dispatch, cross-module overload lookup, and runtime overload selection remain
deferred.

## Consequences

- Overloading and overriding remain distinct and share one candidate/dispatch
  pipeline.
- Exact overloads for currently accepted primitive, string, array, class, and
  interface types can use the task-005 ABI without implicit conversions.
- Ambiguity is explicit and source mapped, rather than dependent on source
  order or backend representation.
- No public symbol mangling, object layout, vtable layout, or FFI contract is
  introduced.

## Dependencies

ADR-0026, ADR-0041, ADR-0062, ADR-0065, ADR-0066, ADR-0067, ADR-0069,
ADR-0070, ADR-0073, ADR-0074, and ADR-0075.

## Supersession

This ADR supersedes only the overload-deferral portions of ADR-0026,
ADR-0041, ADR-0067, and ADR-0069. It does not authorize secondary
constructors, cross-module lookup, generic overloads, or any change to final,
override, ownership, or ABI rules.
