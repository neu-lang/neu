# ADR-0025 main-task build check Review

Decision: request-revision-before-acceptance

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0025-module-package-visibility-model.md`
- Related ambiguity: `docs/ambiguities/M0014-module-package-visibility-model.md`
- Related milestone: `docs/milestones/M0014-module-package-and-visibility-model.md`

## Findings

The proposal appropriately avoids inventing a package manager or manifest syntax.

Acceptance still needs a deterministic compiler invocation contract. A module name supplied by "tests, command line, or future manifest data" leaves too many implementation choices open.

Target packs and cross compilation are deferred, but ADR-0020 means module identity should not encode host file paths or host-specific source roots. The accepted rule must explicitly prohibit host-path-derived identity for artifacts.

Artifact compatibility is deferred, but the accepted M0014 model should identify which fields are stable enough to be carried forward into later artifact metadata.

## Required Revisions

- Define the bootstrap compiler invocation input for module name.
- Define allowed module name spelling and normalization.
- Define deterministic test module identity construction.
- State that host paths are not module identity.
- State which module metadata fields are frontend-only and which may later appear in target packs.
- Defer artifact compatibility explicitly to M0031 without leaving M0014 implementation ambiguous.

## Non-Authority Reminder

This review does not accept ADR-0025. M0014 implementation remains blocked.
