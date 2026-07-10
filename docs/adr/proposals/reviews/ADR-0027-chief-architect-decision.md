# ADR-0027 Chief Architect Decision

## Metadata

- ADR: `ADR-0027`
- Milestone: `M0018`
- Decider: `Chief Architect`
- Date: `2026-07-10`
- Decision: pending revision

## Decision

ADR-0027 remains a draft proposal and is not accepted source of truth.

Do not implement M0018 type checking from this proposal, from the review artifacts, or from current compiler behavior.

## Required Before Acceptance

- Resolve the typed output shape.
- Define or explicitly defer primitive scalar categories.
- Define literal typing if literals are included.
- Define assignment compatibility, especially nullable assignment.
- Define whether direct function calls are included or deferred.
- Define whether structural function type application is included or deferred.
- Define diagnostic primary spans, recovery actions, source-of-truth citations, and safe suggestion policies.
- Update the ambiguity report only after accepted source of truth exists.

## Rationale

The proposal is useful direction, but the current text is still too broad and contains conditional wording that would let implementation agents guess important semantics.

## Boundary

M0018 remains blocked for well-typed and ill-typed fixture implementation until ADR-0027 is revised and accepted.
