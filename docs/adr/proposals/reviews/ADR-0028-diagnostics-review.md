# ADR-0028 Diagnostics Review

## Metadata

- Proposal: `ADR-0028`
- Milestone: `M0019`
- main-task review: `main-task diagnostics check`
- Date: `2026-07-10`
- Decision: `request revision before acceptance`

## Inputs Read

- `docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md`
- `docs/ambiguities/M0019-nullability-and-flow-typing.md`
- `docs/SPEC.md`
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- `docs/adr/ADR-0027-type-checking-core.md`

## Findings

No blocking diagnostic concept is missing, but the accepted ADR must tighten several obligations.

## Required Diagnostic Revisions

- `invalid_nullable_use` must define the primary span for name expressions, assignment values, and grouped expressions.
- `invalidated_refinement` must define whether the primary span is the invalidating operation, the later use, or both through primary and secondary spans.
- `unsupported_flow_rule` must define stable rule identifiers for unsupported boolean combinations, member refinements, calls, mutable locals, parameters, and exclusive-borrow refinements.
- `ambiguous_flow_rule` must define recovery action when name resolution is ambiguous versus when control-flow interpretation is ambiguous.
- Every diagnostic must define a recovery action, source-of-truth citation, and safe suggestion policy before acceptance.
- Diagnostics must not recommend force unwraps, unsafe casts, hidden copies, or mutation changes.

## Diagnostic Acceptance Bar

Before ADR-0028 is accepted, it should include stable diagnostic identifiers and fixture-facing rule names for at least:

- `invalid_nullable_use`
- `invalidated_refinement`
- `unsupported_flow_rule`
- `ambiguous_flow_rule`

## Decision

Request revision before acceptance. The draft is directionally compatible with ADR-0015, but diagnostic details are not yet concrete enough for implementation.
