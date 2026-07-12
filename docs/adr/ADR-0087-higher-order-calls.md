# ADR-0087: Higher-Order Calls

## Status

Accepted.

## Decision

An indirect call requires an exact non-null function type. The callee value is
evaluated before its arguments, and arguments are evaluated left to right.
Parameter and return types must match exactly; no numeric conversion,
overload-by-return-type, nullable function value, or implicit function
conversion is allowed.

Named top-level function values retain their declaration identity through HIR,
MIR, and Cranelift. Their inferred ownership/effect contract applies at the
indirect call site. Function values are copyable, `Send`, and `Share` under
ADR-0086. Return ownership is the referenced function's declared contract.

Indirect calls use compiler-private target-pack signatures and function
addresses. No public pointer, stable function layout, FFI representation,
reflection, closure capture, or callback crossing an unapproved boundary is
introduced. Generic function values require explicit specialization arguments
under ADR-0085.

## Consequences

HIR and MIR must distinguish function references and indirect calls from direct
declaration calls. Diagnostics are emitted before backend lowering for invalid
callee type, arity, argument type, visibility, or stale effect metadata.

## Dependencies

ADR-0041, ADR-0062, ADR-0075, ADR-0085, ADR-0086.
