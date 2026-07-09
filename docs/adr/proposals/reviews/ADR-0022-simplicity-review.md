# ADR-0022 Simplicity Review

Decision: request-revision-before-acceptance

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0022-declaration-syntax.md`
- Related milestone: `docs/milestones/M0011-declaration-parser.md`

## Simplicity Findings

The recommended direction is appropriate because it chooses a small Kotlin-like custom declaration grammar and rejects adopting Kotlin wholesale.

Before acceptance, the grammar should stay narrow:

- accept only declarations required by near-term parser milestones
- defer annotations
- defer companion-like declarations
- defer primary-constructor sugar unless it is required by the type model
- defer declaration modifiers other than visibility unless explicitly justified
- avoid property semantics while expression and type syntax remain unresolved

## Non-Authority Finding

This review supports the proposal direction but does not accept declaration syntax. Parser implementation remains blocked.
