# ADR-0041: Bootstrap Function Call And Return Semantics

Status: Accepted

## Question

What function call and return semantics are accepted for the first executable
subset?

## Competing Designs

1. Direct top-level calls only.
2. General overload resolution and function values.
3. Inline-only function calls.
4. Backend-invented call semantics.

## Trade-offs

Direct top-level calls are enough for an adding-two-integers executable and
reuse existing name resolution boundaries.

General overload resolution and function values are expressive, but require
semantics not yet accepted.

Inline-only calls reduce ABI work but would hide the function-call contract
from HIR, MIR, and backend validation.

Backend-invented calls violate the source-of-truth rule.

## Recommended Choice

The bootstrap executable subset accepts direct calls to resolved top-level
functions in the same module/package. A call expression is well-typed when:

- the callee resolves to exactly one function declaration;
- the callee has a body or is the current function during direct recursion
  rejection checks;
- the number of arguments equals the number of parameters;
- each argument expression type is assignment-compatible with the corresponding
  parameter type under ADR-0027; and
- the call expression type is the callee's declared return type.

The bootstrap subset rejects direct recursion and mutually recursive calls
because stack behavior and recursion diagnostics are not yet specified.

Arguments are evaluated left-to-right before entering the callee. For bootstrap
values, parameter passing is by value. `Bool`, `Int`, `Unit`, and `Null` are
copyable under ADR-0035. Only `Int` participates in the arithmetic bootstrap
smoke. Owned `String` calls are additionally accepted exactly as specified by
ADR-0064; user-defined move-only values remain out of the first executable
subset.

Only explicit `return expression;` returns a value from a function in the first
executable subset. Implicit returns from trailing expressions are deferred for
runtime semantics, even if parsed. A function declared to return `Int` must
have an explicit `return` on every reachable path accepted by the bootstrap
control-flow subset. `return;` is accepted only for functions declared `Unit`
and is not part of the first executable smoke.

Diagnostics:

- `invalid_call_target` for unresolved, ambiguous, non-function, external, or
  unsupported callee forms;
- `argument_count_mismatch`;
- `argument_type_mismatch`;
- `recursive_call_unsupported`;
- `return_type_mismatch`;
- `missing_return`;
- `unreachable_return` when a return statement cannot be reached within the
  accepted bootstrap control-flow subset.

## Downstream Consequences

- HIR must preserve direct callee identity, argument order, and return type.
- MIR must preserve left-to-right argument evaluation.
- Backend smoke tests may call arithmetic helper functions.
- Overloads, method calls, function values, closures, recursion, default
  arguments, named arguments, varargs, and generic calls remain deferred.

## Dependencies

- ADR-0026
- ADR-0027
- ADR-0035
- ADR-0040
- ADR-0042
