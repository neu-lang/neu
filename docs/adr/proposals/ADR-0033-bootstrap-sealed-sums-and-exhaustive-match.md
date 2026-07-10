# ADR-0033: Bootstrap Sealed Sums And Exhaustive Match

Status: Accepted as `docs/adr/ADR-0033-bootstrap-sealed-sums-and-exhaustive-match.md`

## Non-Authority Notice

This proposal is retained as design history. The accepted ADR is the only
implementation authority.

## Question

What smallest sealed-sum and match subset can M0021 implement without assuming
Kotlin `when` semantics, payload destructuring, or open hierarchy behavior?

## Competing Designs

1. Add closed no-payload `enum` variants and an expression-level `when` form
   with qualified case patterns.
2. Add payload enum variants and destructuring patterns in the same milestone.
3. Use sealed interfaces with independently declared variants.
4. Defer all algebraic-data and matching behavior.

## Trade-offs

No-payload enum variants give a concrete finite coverage set with small parser
and diagnostic surface. They do not cover result payloads or data-carrying
domain models yet.

Payload variants and destructuring are more useful, but require field,
constructor, binding, ownership, and type-substitution rules that are not
accepted.

Sealed interfaces support flexible hierarchies but need module-level sealed
scope and subtype-resolution rules. Deferral preserves simplicity but leaves
ADR-0012 without an implementable subset.

## Recommended Draft Choice

Choose option 1 for M0021:

- `enum` declares a closed sum in its declaring module/package scope.
- An enum body contains zero or more identifier-only variants, separated by
  commas or semicolons; variants have no payloads, fields, or constructors.
- `when` is an expression with a subject expression and braced arms of the
  form `QualifiedVariant -> expression`, plus an optional `_ -> expression`
  wildcard arm.
- A match is exhaustive only when each declared variant appears exactly once,
  or one wildcard arm appears. Duplicate or unknown variants are errors.
- Exhaustiveness is evaluated only after exact enum-subject resolution; other
  subjects and all payload/destructuring forms remain deferred.

The accepted ADR must define exact grammar, scope identity, diagnostic names,
spans, recovery, wildcard interaction, and source-order behavior. It must not
infer Kotlin `when` semantics, implicit smart casts, or ownership behavior.

## Dependencies

ADR-0006, ADR-0007, ADR-0011, ADR-0012, ADR-0015, ADR-0017, ADR-0022,
ADR-0024, M0021.

## Handoff

Language Designer should refine the concrete grammar and diagnostics; Chief
Architect must approve an accepted ADR and matching `SPEC.md` revision before
implementation begins.
