# ADR-0026 Chief Architect Decision

Decision: approved

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0026-name-resolution-policy.md`
- Related ambiguity: `docs/ambiguities/M0016-name-resolution-policy.md`
- Related milestone: `docs/milestones/M0016-name-resolution-pass.md`

## Current Decision

ADR-0026 is accepted.

The accepted source of truth is `docs/adr/ADR-0026-name-resolution-policy.md`, with the summary incorporated into `docs/SPEC.md`.

M0016 ambiguity is resolved. Implementation may proceed only against the accepted ADR-0026 model.

## Required Review Dependencies

- Language Lawyer review.
- Diagnostics Engineer review.
- Adversarial Engineer review.
- Spec Compliance Auditor review.
- Simplicity Guardian review.

## Resolved Acceptance Blockers

- Exact resolvable AST node kinds.
- Exact declaration and local binding positions.
- Pattern binding exclusion for M0016.
- Declaration order and local-before-declaration behavior.
- Shadowing rules.
- Duplicate-name behavior.
- Ambiguous-name behavior.
- Unresolved-name diagnostics.
- Visibility enforcement deferral.
- Explicit rejection or deferral of imports, cross-module lookup, members, overloads, extensions, and type-directed lookup.

## Consequences

- `docs/ambiguities/M0016-name-resolution-policy.md` is resolved.
- M0016 name-resolution implementation may proceed only for accepted ADR-0026 semantics.
- Imports, cross-module lookup, members, overloads, extensions, and type-directed lookup remain deferred.
