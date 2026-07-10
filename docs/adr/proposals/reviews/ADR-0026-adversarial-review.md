# ADR-0026 Adversarial Engineer Review

Decision: request-revision-before-acceptance

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0026-name-resolution-policy.md`
- Related ambiguity: `docs/ambiguities/M0016-name-resolution-policy.md`
- Related milestone: `docs/milestones/M0016-name-resolution-pass.md`

## Findings

The largest soundness risk is silent misbinding. If lookup order, shadowing, duplicate behavior, or ambiguity handling is underspecified, later type checking and ownership analysis may reason about the wrong declaration.

The draft must explicitly prevent imports from becoming active by accident. Parser support for import syntax must not imply import semantics.

Duplicate top-level and local names need a precise policy. Reporting a duplicate hook in M0015 is not enough; M0016 must decide when duplicate input becomes an error and when distinct declarations are allowed.

Ambiguity handling must reject ambiguous candidates rather than select by insertion order or source-file order unless that order is explicitly accepted.

## Required Revisions

- Define shadowing rules that cannot accidentally select a less local declaration.
- Define duplicate local and top-level behavior.
- Define ambiguity reporting when multiple candidates survive lookup.
- Define source-file order effects, if any.
- Explicitly state that imports, cross-module lookup, and member lookup are rejected or deferred in M0016.

## Source-Of-Truth Boundary

This review does not accept ADR-0026.

M0016 implementation must remain blocked until these risks are resolved in accepted source of truth.
