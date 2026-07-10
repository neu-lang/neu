# ADR-0025 main-task language review Review

Decision: request-revision-before-acceptance

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0025-module-package-visibility-model.md`
- Related ambiguity: `docs/ambiguities/M0014-module-package-visibility-model.md`
- Related milestone: `docs/milestones/M0014-module-package-and-visibility-model.md`

## Findings

The proposal correctly preserves non-authority status and identifies that ADR-0017 is too high-level for implementation.

It is not ready for acceptance because `private` visibility remains undecided. The accepted model must say whether `private` is file-private, declaration-private, package-private, type-member-private, or some smaller bootstrap subset.

Default visibility is also unresolved. Parser syntax permits omitted visibility, so M0014 cannot represent complete visibility metadata until omitted visibility has a specified value or an explicit unresolved state.

Module identity needs a concrete accepted input contract. "Supplied explicitly by tests, command line, or future manifest data" is directionally useful, but implementation needs a single bootstrap rule.

Package declaration absence behavior must be concrete. The proposal says root package "if accepted by review"; acceptance must decide the rule.

## Required Revisions

- Define exact `private` visibility semantics for the bootstrap frontend.
- Define default visibility for omitted visibility modifiers.
- Define module identity input format for bootstrap tests and compiler invocation.
- Define source file to module assignment rules.
- Define package absence and root package behavior.
- Define whether visibility metadata attaches only to declarations.
- Define module model diagnostics with primary span or external input location.

## Non-Authority Reminder

This review does not accept ADR-0025. M0014 implementation remains blocked.
