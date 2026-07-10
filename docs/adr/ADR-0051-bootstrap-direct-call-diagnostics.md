# ADR-0051: Bootstrap Direct Call Diagnostics

Status: Accepted

## Question

Where do bootstrap direct-call diagnostics attach, and what recovery applies?

## Competing Designs

1. Attach every diagnostic to the whole call expression.
2. Attach each diagnostic to the narrowest relevant source form.
3. Defer calls until HIR.

## Recommended Choice

- `invalid_call_target` attaches to the callee expression; recovery gives the
  call no successful target or result type.
- `argument_count_mismatch` attaches to the call expression; recovery gives no
  successful call result type.
- `argument_type_mismatch` attaches to the mismatching argument expression;
  recovery gives no successful call result type.
- `recursive_call_unsupported` attaches to the recursive call expression;
  recovery gives no successful call result type.

All source locations are source-file-qualified spans. Suggestions are limited
to calling an accessible same-package top-level function, matching its arity
and parameter types, or removing recursion. This ADR does not change call
eligibility, evaluation order, or parameter passing from ADR-0041.

## Dependencies

- ADR-0015
- ADR-0026
- ADR-0041
