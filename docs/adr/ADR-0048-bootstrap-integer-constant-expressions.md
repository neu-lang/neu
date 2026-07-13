# ADR-0048: Bootstrap Integer Constant Expressions

Status: Accepted

## Question

What expressions establish the statically known facts required by ADR-0043?

## Competing Designs

1. Literal expression trees only.
2. Literal trees plus immutable local bindings.
3. Any compiler-evaluable expression, including calls.

## Trade-offs

Literal trees are deterministic and local. The broader alternatives introduce
unapproved binding initialization, call, recursion, ordering, and ownership
evaluation semantics.

## Recommended Choice

A bootstrap integer constant expression is exactly an integer literal or a
grouped, unary `+`/`-`/`~`, or binary ADR-0042 integer-operator expression
whose operands are bootstrap integer constant expressions. A literal whose
magnitude exceeds `u64` has no successful value but remains available for the
`integer_literal_out_of_range` diagnostic.

Local bindings, names, assignments, calls, member access, `if`, `when`, and
all other forms are not bootstrap integer constant expressions. They do not
receive ADR-0043 static arithmetic diagnostics merely because a runtime value
might be inferred.

## Downstream Consequences

- The compiler evaluates only literal, grouped, unary, and accepted binary metadata.
- Runtime failures outside this boundary trap under ADR-0043.

## Dependencies

- ADR-0042
- ADR-0043
