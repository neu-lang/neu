# ADR-0026 Language Lawyer Review

Decision: request-revision-before-acceptance

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0026-name-resolution-policy.md`
- Related ambiguity: `docs/ambiguities/M0016-name-resolution-policy.md`
- Related milestone: `docs/milestones/M0016-name-resolution-pass.md`

## Findings

ADR-0026 is a useful direction, but it is not ready for acceptance because the exact AST node kinds that contain resolvable names are not enumerated.

The proposal must define whether pattern bindings participate in M0016. It currently defers them unless a later accepted ADR defines binding positions, but ADR-0024 already includes pattern syntax. Acceptance must either explicitly exclude pattern bindings from M0016 or define the exact accepted binding behavior.

The proposal must define declaration order. Top-level and local lookup cannot be implemented until the language states whether references may see later declarations in the same package, declaration body, or block.

The proposal must define shadowing between local names, top-level names, package-qualified names, and nested declaration bodies.

## Required Revisions

- Enumerate exact AST node kinds that introduce declarations and references.
- Define whether pattern bindings are excluded from M0016 or included with exact binding rules.
- Define declaration order for top-level declarations, declaration bodies, and blocks.
- Define local-before-declaration behavior.
- Define shadowing rules for local and top-level names.
- Define whether visibility is checked during M0016 or deferred.

## Source-Of-Truth Boundary

This review does not accept ADR-0026.

No implementation may depend on ADR-0026 until accepted source of truth exists.
