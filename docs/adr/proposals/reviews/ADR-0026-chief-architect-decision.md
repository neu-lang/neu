# ADR-0026 Chief Architect Decision

Decision: pending-revision

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0026-name-resolution-policy.md`
- Related ambiguity: `docs/ambiguities/M0016-name-resolution-policy.md`
- Related milestone: `docs/milestones/M0016-name-resolution-pass.md`

## Current Decision

ADR-0026 is not accepted.

M0016 remains blocked for implementation until the proposal is revised into concrete accepted source of truth.

## Required Review Dependencies

- Language Lawyer review.
- Diagnostics Engineer review.
- Adversarial Engineer review.
- Spec Compliance Auditor review.
- Simplicity Guardian review.

## Acceptance Blockers

- Exact resolvable AST node kinds.
- Exact declaration and local binding positions.
- Pattern binding inclusion or exclusion.
- Declaration order and local-before-declaration behavior.
- Shadowing rules.
- Duplicate-name behavior.
- Ambiguous-name behavior.
- Unresolved-name diagnostics.
- Visibility enforcement decision.
- Explicit rejection or deferral of imports, cross-module lookup, members, overloads, extensions, and type-directed lookup.

## Consequences

- `docs/ambiguities/M0016-name-resolution-policy.md` remains open.
- M0016 name-resolution implementation may not proceed.
- The next safe task is to revise ADR-0026 into a concrete draft model.
