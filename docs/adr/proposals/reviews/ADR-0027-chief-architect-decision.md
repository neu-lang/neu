# ADR-0027 main task Decision

## Metadata

- ADR: `ADR-0027`
- Milestone: `M0018`
- Decider: `main task`
- Date: `2026-07-10`
- Decision: approved

## Decision

ADR-0027 is accepted.

The accepted source of truth is `docs/adr/ADR-0027-type-checking-core.md`, with the summary incorporated into `docs/SPEC.md`.

M0018 ambiguity is resolved. Implementation may proceed only against the accepted ADR-0027 model.

## Resolved Acceptance Blockers

- Resolve the typed output shape.
- Define or explicitly defer primitive scalar categories.
- Define literal typing if literals are included.
- Define assignment compatibility, especially nullable assignment.
- Define whether direct function calls are included or deferred.
- Define whether structural function type application is included or deferred.
- Define diagnostic primary spans, recovery actions, source-of-truth citations, and safe suggestion policies.
- Resolve the M0018 ambiguity report.

## Rationale

The concrete draft is narrow enough for M0018 implementation and explicitly defers calls, function type application, overloads, conversions, generic solving, ownership, borrow checking, HIR, MIR, and backend behavior.

## Boundary

M0018 well-typed and ill-typed fixture implementation may proceed only for accepted ADR-0027 semantics.
