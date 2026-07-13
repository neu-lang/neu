# ADR-0033: Bootstrap Sealed Sums And Exhaustive Match

Status: Accepted

## Question

What smallest sealed-sum and match subset can the compiler implement without assuming
Kotlin `when` semantics, payload destructuring, or open hierarchy behavior?

## Competing Designs

1. Add closed no-payload `enum` variants and an expression-level `when` form
   with qualified case patterns.
2. Add payload enum variants and destructuring patterns in the same implementation phase.
3. Use sealed interfaces with independently declared variants.
4. Defer all algebraic-data and matching behavior.

## Decision

Choose option 1. The bootstrap `enum` keyword declares a closed sum in its
declaring module/package scope. Its body contains zero or more identifier-only
variants, separated by commas or semicolons; variants have no payloads, fields,
constructors, visibility modifiers, or nested declarations.

`when` is an expression:

```text
when-expression = `when` `(` expression `)` `{` match-arm* `}`
match-arm = match-pattern `->` expression `;`?
match-pattern = qualified-variant-pattern | `_`
qualified-variant-pattern = identifier `.` identifier
```

The subject must resolve to exactly one declared bootstrap enum. A qualified
variant pattern must resolve to one variant of that enum; unqualified variants,
literal patterns, binding patterns, grouped patterns, payload patterns, and
patterns from another enum are rejected for this subset.

A `when` expression is exhaustive when every declared variant is covered once,
or exactly one wildcard arm is present. Duplicate variant arms, multiple
wildcard arms, unknown variants, and missing variants are errors. Arm bodies
are ordinary accepted expressions and do not gain implicit smart casts or
binding semantics.

## Diagnostics And Recovery

Diagnostics use the pattern or subject node as their primary span:

- `duplicate_enum_variant` for a repeated enum variant name;
- `unknown_match_variant` for a qualified variant not declared by the subject
  enum;
- `duplicate_match_variant` for a repeated variant arm;
- `duplicate_match_wildcard` for a second wildcard arm;
- `non_exhaustive_match` for an otherwise valid match missing one or more
  variants;
- `invalid_match_subject` when subject resolution does not identify an enum.

Malformed syntax retains ordinary parser diagnostics and recovers at an arm
semicolon, right brace, declaration boundary, or end of file as applicable.
Semantic diagnostics preserve independently valid arms and continue checking.

## Consequences And Deferrals

This establishes a finite, module/package-scoped coverage set for this implementation. It
does not define payload variants, destructuring, enum methods, sealed
interfaces, open hierarchies, generic enums, nullable match coverage, type
unification across arm results, expression evaluation, ownership behavior, or
smart-cast behavior. Those need later accepted decisions.

## Dependencies And Supersession

Depends on ADR-0006, ADR-0007, ADR-0011, ADR-0012, ADR-0015, ADR-0017,
ADR-0022 and ADR-0024. This narrowly supersedes ADR-0022's enum
variant deferral and ADR-0024's match/`when` deferral for the exact bootstrap
subset above. It resolves the corresponding ambiguity report.
