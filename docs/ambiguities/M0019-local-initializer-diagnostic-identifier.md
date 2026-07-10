# Ambiguity Report: M0019 Local Initializer Diagnostic Identifier

## Metadata

- Report ID: `M0019-local-initializer-diagnostic-identifier`
- Related Task: `M0019-015`
- Related Milestone: `M0019`
- Filed By: `Language Lawyer`
- Date: `2026-07-10`
- Status: `resolved`
- Required Owner: `Language Designer`

## Ambiguous Or Missing Authority

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
  - `docs/adr/ADR-0029-immutable-local-const-keyword.md`
- Milestone:
  - `docs/milestones/M0019-nullability-and-flow-typing.md`

## Exact Ambiguous Text Or Missing Rule

```text
ADR-0028, Nullable Use Rules: "assigning T? to T without an active non-null
refinement reports invalid_nullable_use" and "using a name expression of type
T? where T is expected without an active non-null refinement reports
invalid_nullable_use."

ADR-0028, Flow Diagnostics: "Required stable rule identifier examples:
nullable_value_without_refinement, nullable_assignment_without_refinement."

ADR-0027 gives the initializer expression as the primary span for a declaration
annotation mismatch, but does not classify a declaration initializer as an
assignment for diagnostic-identifier selection.

Missing rule: which stable rule identifier an unrefined `T?` name expression
must use when it is the initializer of `const definite: T = maybe`.
```

## Competing Interpretations

1. Use `nullable_assignment_without_refinement` because the initializer is checked for compatibility with the annotated target type.
2. Use `nullable_value_without_refinement` because the failing operand is a nullable name expression used where `T` is expected.
3. Use another stable identifier because ADR-0028 labels the two listed identifiers as examples and provides no initializer-specific mapping.

## Why Guessing Is Unsafe

- Stable rule identifiers are fixture-facing diagnostic obligations; selecting one would make tests and later implementation establish an uncited diagnostic contract.
- ADR-0028 fixes the diagnostic family and span but does not resolve the identifier choice for declaration initializers.

## Affected Work

- Tasks blocked:
  - `M0019-015`
- Milestones affected:
  - `M0019`
- Tests blocked:
  - Local-initializer diagnostic expectations for unrefined `T? -> T`.
- Implementation areas blocked:
  - Refinement-aware local-initializer diagnostic emission.

## Recommended Resolution Path

- [x] Language Lawyer determines whether existing text resolves it.
- [x] Language Designer drafts ADR and spec revision.
- [x] Adversarial Engineer reviews soundness risk.
- [x] Diagnostics Engineer reviews diagnostic consequences.
- [x] Simplicity Guardian reviews complexity.
- [x] Chief Architect approves final resolution.

## Temporary Rule

No implementation or test may select a stable diagnostic identifier for this local-initializer case until accepted authority maps the case to an identifier or declares the mapping non-blocking.

## Resolution

- Decision:
  - ADR-0030 maps the exact bare nullable-name `T?` local initializer to an
    annotated base type `T` to `invalid_nullable_use` with stable rule
    identifier `nullable_assignment_without_refinement`.
- Source of truth updated:
  - `docs/adr/ADR-0030-local-initializer-nullable-diagnostic.md`
  - `docs/SPEC.md` ADR-0028 M0019 summary
- Date resolved:
  - `2026-07-10`
