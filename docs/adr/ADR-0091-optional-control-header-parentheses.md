# ADR-0091: Optional Control-Header Parentheses

## Status

Accepted.

## Decision

Parentheses are optional only around `if`, `for`, and `when` headers. The
parenthesized forms remain valid, and parentheses continue to provide ordinary
expression grouping. An unparenthesized header ends at the opening brace that
starts its required body or arm block. The existing expression parser consumes
the condition or subject up to that boundary; no new expression or loop kind
is introduced.

`for` keeps its existing range and array-iteration grammar, including the
required `in` separator. `when` keeps its existing subject and arm grammar.
`else` attachment, ownership, nullability, exhaustiveness, loop controls,
cleanup, and source spans are unchanged. Both spellings normalize to the same
AST, HIR, MIR, and backend facts.

Malformed or ambiguous headers are diagnostics before HIR lowering. Function
calls, grouped expressions inside headers, multiline expressions, comments,
and optional semicolons retain their existing meaning. Parentheses are still
required wherever they are needed to disambiguate an ordinary expression from
the control-header boundary.

## Consequences

The lexer needs no new token. Parser recovery must recognize the body brace as
the unparenthesized header boundary and preserve existing spans. Semantic and
backend stages consume normalized control-flow metadata, so runtime and ABI
behavior do not change.

## Dependencies

ADR-0021, ADR-0024, ADR-0060, ADR-0063, ADR-0077.

## Deferred

`while`, labels, custom loop steps, new patterns, new loop kinds, and removal
of parentheses from function calls or general expressions remain deferred.
