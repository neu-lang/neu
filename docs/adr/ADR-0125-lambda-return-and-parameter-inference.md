# ADR-0125: Lambda Return And Parameter Inference

Status: Accepted; decided by the project owner on neu-lang/neu#84

## Question

An unannotated lambda local, such as `val f = { -> 7 };`, is not callable,
because no rule gives its return type. Should a lambda's return type be
inferred from its body, and may a lambda parameter omit its type?

## Decision

- A lambda's return type is the type of its single body expression, as
  ADR-0088 and SPEC (Lambda Bodies) require. `{ -> 7 }` has type `() -> Int`.
- A lambda parameter may omit its type annotation. Its type comes from the
  expected function type at the lambda's position, in this order:
  1. the annotation of the local that the lambda initializes;
  2. the annotated parameter at the same position in a callee that has exactly
     one top-level function declaration with that name.
  An overloaded callee provides no context.
- A lambda parameter that has no annotation and no expected type is an error,
  `lambda parameter type required`, reported at the lambda.
- A local initialized by a lambda expression is function-typed. It may be
  called whether or not it has an annotation.

Lambda bodies remain single expressions. Generic lambdas, expected types from
return positions, and context from overloaded callees are not part of this ADR.

## Consequences

Type checking and HIR use one predicate for function-typed locals, so a call
that type checks also lowers. Local inference (ADR-0090) and the capture rules
(ADR-0088, ADR-0089) are unchanged.
