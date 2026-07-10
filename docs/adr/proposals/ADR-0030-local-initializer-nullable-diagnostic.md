# ADR-0030: Local Initializer Nullable Diagnostic Identifier

Status: Draft proposal — non-authoritative

## Non-Authority Notice

This proposal is not accepted source of truth and must not drive implementation,
tests, or `docs/SPEC.md` changes until Chief Architect approval.

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

## Identifier-Mapping Choices

1. **Assignment mapping:** use `nullable_assignment_without_refinement`.
   The initializer is checked against the declared target type, matching the
   task's assignment-compatibility path. This gives declarations and ordinary
   assignments the same target-compatibility identifier.
2. **Value mapping:** use `nullable_value_without_refinement`.
   The offending operand is the nullable name expression. This groups the case
   with other expression-context nullable uses, but distinguishes two otherwise
   identical `T? -> T` compatibility failures by enclosing syntax.
3. **New initializer identifier:** add a declaration-specific identifier. This
   is maximally explicit but expands fixture-facing diagnostic surface without a
   semantic or diagnostic-recovery difference.

## Recommendation

Choose assignment mapping. For an unrefined `T?` name expression checked as an
annotated local initializer for `T`, emit `invalid_nullable_use` with stable
identifier `nullable_assignment_without_refinement`; retain the initializer
expression as the primary span, expected `T`, actual `T?`, and ADR-0028
recovery and suggestion policy.

## Exact Scope And Consequences

This maps one already-rejected local-initializer case to an existing identifier.
It changes no acceptance rule, refinement provenance, span, recovery, wording,
`const` semantics, memory-safety rule, or thread-safety rule. `Null -> T` and
unrelated mismatches remain ADR-0027 `type_mismatch` cases.

Diagnostics gain one deterministic fixture contract and no new category or
identifier. The rejected new-identifier alternative avoids needless diagnostic
taxonomy; the value mapping is less consistent with the task's declared-target
compatibility check.

## Supersession And Acceptance Targets

This proposal supersedes no accepted authority. If accepted, ADR-0030 clarifies
only ADR-0028's `invalid_nullable_use` stable-identifier examples for annotated
local initializers; ADR-0027's initializer-span rule and ADR-0029's spelling
rule remain unchanged. Acceptance requires Diagnostics Engineer, Adversarial
Engineer, Simplicity Guardian, and Chief Architect review. Afterwards, the
ADR-0028 summary in `docs/SPEC.md` and M0019-015 diagnostic expectation may be
updated through their stated workflows.
