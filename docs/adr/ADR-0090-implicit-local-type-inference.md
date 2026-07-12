# ADR-0090: Implicit Local Type Inference

## Status

Accepted.

## Decision

`val` and `var` local declarations may omit their type annotation only when
they have an initializer. The compiler infers the initializer's exact static
type and records it as the binding type. Inference is local-only: parameters,
returns, fields, constructor fields, and public signatures still require
annotations.

Inference never widens a concrete class to an interface or superclass, never
performs numeric conversion, and never selects a common supertype. A bare
`null` initializer is rejected because it has no deterministic nullable base;
an explicit nullable annotation remains required. Calls, `new`, arrays,
strings, enums, generic instances, function values, and accepted conditional
expressions infer their already-resolved result type.

An inferred `var` retains its inferred type for its whole scope. Later
assignments must satisfy the existing assignment-compatibility rule and cannot
change the binding type. Ownership, borrowing, nullability, capability,
dispatch, cleanup, and source mapping facts are identical to an explicitly
annotated local. Declaration order and shadowing use existing local-binding
identity; unresolved or ambiguous initializer types are diagnostics.

## Diagnostics And Boundaries

Missing initializers, bare-null inference, unresolved initializer types, and
ambiguous inference are diagnosed at the declaration or initializer span before
HIR lowering. Inferred types are internal semantic facts and do not change the
ABI or module signature surface.

## Consequences

The parser retains optional annotations, while type checking must resolve
initializer expressions before finalizing local declaration signatures. HIR
and MIR consume the same resolved `TypeId` as explicit locals; no backend type
or representation may determine source inference.

## Dependencies

ADR-0016, ADR-0027, ADR-0028, ADR-0035, ADR-0062, ADR-0063, ADR-0064,
ADR-0082, ADR-0087.

## Deferred

Inference for public declarations, common-supertypes, union types, implicit
conversions, numeric widening, null insertion, generic constraint solving,
overload-based inference, and runtime type tests remain deferred.
