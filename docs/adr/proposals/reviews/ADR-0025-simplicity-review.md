# ADR-0025 Simplicity Review

Decision: request-revision-before-acceptance

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0025-module-package-visibility-model.md`
- Related ambiguity: `docs/ambiguities/M0014-module-package-visibility-model.md`
- Related milestone: `docs/milestones/M0014-module-package-and-visibility-model.md`

## Findings

The proposed small frontend module model is the right size for M0014.

The draft should defer more packaging concepts from the accepted core model. Manifest references, target-pack artifact keys, and module dependency metadata can be acknowledged as future work, but the accepted M0014 implementation should not need them.

The proposal should choose one bootstrap path: explicit module name plus package namespaces. Keeping multiple identity sources in the accepted text would invite unnecessary abstraction.

## Required Revisions

- Choose explicit module name as the sole bootstrap module identity input.
- Keep manifests and package manager behavior deferred.
- Keep module dependencies deferred until name resolution or build graph work.
- Define a minimal visibility metadata record.
- Defer re-exports, friend modules, protected visibility, sealed scope, extension scope, and protocol conformance scope.

## Non-Authority Reminder

This review does not accept ADR-0025. M0014 implementation remains blocked.
