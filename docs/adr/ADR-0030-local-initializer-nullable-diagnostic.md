# ADR-0030: Local Initializer Nullable Diagnostic Identifier

Status: Accepted

## Question

Which stable rule identifier applies when an annotated local initializer is an
unrefined nullable name expression, for example `const definite: T = maybe`
where `maybe: T?`?

## Existing Authority

ADR-0028 requires `invalid_nullable_use` for both assigning `T?` to `T` and
using an unrefined nullable name where `T` is expected. Its flow-diagnostic
paragraph lists `nullable_value_without_refinement` and
`nullable_assignment_without_refinement` as stable identifier examples.
ADR-0027 fixes a declaration annotation mismatch's primary span to its
initializer expression. ADR-0029 changes only immutable-local spelling and
preserves its initializer rules. Neither selects an identifier for this path.

## Competing Designs

1. **Assignment mapping:** use `nullable_assignment_without_refinement`.
   The initializer is checked against the declared target type, matching the
   local-initializer compatibility path.
2. **Value mapping:** use `nullable_value_without_refinement`. The offending
   operand is the nullable name expression used where `T` is expected.
3. **New initializer identifier:** add a declaration-specific identifier.

## Trade-offs

Assignment mapping keeps this target-compatibility failure aligned with
ordinary assignment without adding diagnostic taxonomy. Value mapping
distinguishes otherwise identical `T? -> T` compatibility failures by their
enclosing syntax. A new identifier is explicit, but has no semantic or
diagnostic-recovery benefit.

## Decision

For an annotated local declaration whose initializer is exactly a bare resolved
name expression of nullable type `T?`, where the annotation is the nullable
base type `T`, emit `invalid_nullable_use` with stable rule identifier
`nullable_assignment_without_refinement` when no active non-null refinement
applies.

The primary span remains the initializer expression under ADR-0027. Expected
and actual type displays, recovery, and safe-suggestion policy remain those of
ADR-0028.

## Consequences And Limits

This maps one already-rejected local-initializer case to an existing identifier.
It changes no acceptance rule, refinement provenance, span, recovery, wording,
`const` semantics, memory-safety rule, or thread-safety rule.

This decision applies only to the exact bare nullable-name `T?` initializer to
annotated-base-`T` local-initializer case. It does not select an identifier for
grouped expressions, other expression shapes, other source or target types,
or other contexts. `Null -> T` and unrelated mismatches remain ADR-0027
`type_mismatch` cases.

## Rejected Alternatives

The value mapping is rejected because this narrowly defined path is checked
against the annotated local target. A new initializer identifier is rejected
because it needlessly expands the fixture-facing diagnostic surface.

## Dependencies

- ADR-0027
- ADR-0028
- ADR-0029

## Supersession

This ADR supersedes no accepted authority. It resolves the stable-identifier
mapping left open by ADR-0028 for the exact case stated above. ADR-0027's
initializer-span rule and ADR-0029's immutable-local spelling rule remain
unchanged.
