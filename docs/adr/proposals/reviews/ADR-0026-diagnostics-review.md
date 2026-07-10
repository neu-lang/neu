# ADR-0026 Diagnostics Engineer Review

Decision: request-revision-before-acceptance

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0026-name-resolution-policy.md`
- Related ambiguity: `docs/ambiguities/M0016-name-resolution-policy.md`
- Related milestone: `docs/milestones/M0016-name-resolution-pass.md`

## Findings

The proposal names the right diagnostic categories, but the accepted version must specify each diagnostic with ADR-0015-level detail.

Each resolution diagnostic needs a primary span, recovery action, source-of-truth citation, and safe suggestion policy.

Unresolved-name suggestions are especially risky because imports, cross-module lookup, member lookup, and type-directed lookup are deferred. Suggestions must not imply unavailable lookup paths.

Duplicate-name diagnostics need a primary span policy for the new declaration and a related-location policy for the previous declaration.

Ambiguous-name diagnostics need a way to report candidate locations without exposing compiler-internal table details.

## Required Revisions

- Define `unresolved_name` primary span, recovery action, citation, and safe suggestion policy.
- Define `duplicate_name` primary span and related declaration location policy.
- Define `ambiguous_name` candidate reporting policy.
- Define whether `inaccessible_name` belongs to M0016 or is deferred.
- Define diagnostics for unsupported import, cross-module, and member resolution if those syntaxes are encountered.

## Source-Of-Truth Boundary

This review does not accept ADR-0026.

No diagnostic snapshots for M0016 should be treated as authoritative until the diagnostics are accepted.
