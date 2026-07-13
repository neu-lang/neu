# ADR-0054: Bootstrap Return-Type Mismatch Diagnostics

Status: Accepted

## Question

Where does `return_type_mismatch` attach, and what recovery applies for an
explicit bootstrap return expression?

## Competing Designs

1. Attach to the whole return statement and diagnose unknown values.
2. Attach to the return expression only when both types are known.
3. Attach to the function declaration.

## Trade-offs

The statement and declaration are broader than the incompatible value.
Diagnosing an unknown expression as a mismatch cascades unresolved or deferred
forms. Expression-local known-type checking preserves the existing root cause.

## Recommended Choice

`return_type_mismatch` attaches to the explicit return expression. Emit it only
when the enclosing function's declared return type and the expression type are
both known and incompatible. Recovery records no typed executable return fact
for that statement; the original expression diagnostic remains the only
diagnostic when the expression type is unresolved or deferred.

For the ADR-0042 subset, `Int` is the only value-returning function type.
`return;` and non-`Int` function return behavior remain outside that subset and
are handled by their existing or later diagnostics. This ADR does not alter
ADR-0050 reachability checking.

## Downstream Consequences

- The compiler can validate explicit `Int` returns without diagnostic cascades.
- HIR receives only typed bootstrap return facts.
- Later return inference and broader result types require their own decisions.

## Dependencies

- ADR-0015
- ADR-0027
- ADR-0041
- ADR-0042
- ADR-0050
