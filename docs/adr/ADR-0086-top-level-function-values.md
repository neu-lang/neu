# ADR-0086: Top-Level Function Values

## Status

Accepted.

## Decision

Function types use the existing structural syntax `(T1, T2) -> R` and are
identified by ordered parameter type identities plus return type identity.
Function arguments are types only; receiver function types, named parameters,
effects, and nullable function values remain deferred.

Only named top-level functions with one unambiguous accepted signature may be
converted to a function value. The value is non-null, compiler-managed, and
non-capturing. Overload selection must be explicit through the expected
function type; ambiguous or incompatible references are diagnostics. Function
values may be stored, passed, and returned as opaque compiler-private values.

Application evaluates the function value and arguments left to right and uses
the exact function type. No equality, null value, public function-pointer
layout, FFI representation, reflection, closures, bound methods, or dynamic
loading is introduced. Top-level function values are copyable and satisfy
`Send` and `Share` because they carry no captured state.

HIR and MIR preserve named function identity, function type, source span,
argument order, ownership facts, and indirect-call facts. Cranelift and target
packs use a compiler-private indirect-call representation; the representation
is not a source or FFI ABI.

## Consequences

The type arena gains structural function types. Named references can be
distinguished from local values and direct calls, allowing later higher-order
call and closure tasks to extend the same facts without adding capture
semantics here.

## Dependencies

ADR-0023, ADR-0041, ADR-0075, ADR-0076, ADR-0082, ADR-0085.
