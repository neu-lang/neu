# ADR-0025 Spec Compliance Review

Decision: request-revision-before-acceptance

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0025-module-package-visibility-model.md`
- Related ambiguity: `docs/ambiguities/M0014-module-package-visibility-model.md`
- Related milestone: `docs/milestones/M0014-module-package-and-visibility-model.md`

## Findings

The proposal is correctly labeled as non-authoritative and does not update `docs/SPEC.md` or accepted ADRs.

It is not ready for accepted source of truth because several required accepted content items are still phrased as alternatives or review-dependent choices.

Diagnostics are identified, but acceptance needs concrete diagnostic names and obligations. Each diagnostic must define a primary span or external input location, recovery action, safe suggestion policy, and source-of-truth citation.

The proposal must clarify the boundary with ADR-0022. ADR-0022 defines syntax for `public`, `private`, and `internal`; ADR-0025 must define metadata and semantics for those parsed modifiers.

## Required Revisions

- Convert all review-dependent bullets into concrete accepted rules.
- Add a concrete visibility metadata schema.
- Add concrete diagnostic names.
- Add primary span or external input location for each diagnostic.
- Add recovery action and safe suggestion policy for each diagnostic.
- Clarify which rules are language semantics and which are compiler invocation metadata.

## Non-Authority Reminder

This review does not accept ADR-0025. M0014 implementation remains blocked.
