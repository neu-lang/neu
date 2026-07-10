# ADR-0026 Spec Compliance Auditor Review

Decision: request-revision-before-acceptance

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0026-name-resolution-policy.md`
- Related ambiguity: `docs/ambiguities/M0016-name-resolution-policy.md`
- Related milestone: `docs/milestones/M0016-name-resolution-pass.md`

## Findings

ADR-0026 is correctly marked non-authoritative. It does not update accepted source of truth and therefore cannot unblock implementation.

The proposal must be revised into accepted source of truth before any implementation compares behavior against it.

The accepted version must reconcile ADR-0022 import syntax, ADR-0024 local `val` and `var` syntax, ADR-0025 package/module metadata, and M0015 name-table behavior without treating any existing implementation detail as authority.

## Required Revisions

- Identify exact accepted spec section or ADR destination for the final decision.
- Cite ADR-0015 for diagnostic obligations.
- Cite ADR-0022 for import and declaration syntax boundaries.
- Cite ADR-0024 for local binding syntax boundaries.
- Cite ADR-0025 for module/package/visibility boundaries.
- Define tests that compare against accepted source of truth, not proposal text.

## Source-Of-Truth Boundary

This review does not accept ADR-0026.

`docs/ambiguities/M0016-name-resolution-policy.md` must remain open.
