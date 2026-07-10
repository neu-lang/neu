# ADR-0023 main-task language review Review

Decision: request-revision-before-acceptance

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0023-type-and-generic-syntax.md`
- Related ambiguity: `docs/ambiguities/M0008-type-generic-syntax.md`
- Related milestone: `docs/milestones/M0012-type-and-generic-syntax-parser.md`

## Findings

The proposal identifies the correct decision surface, but it is not yet concrete enough to become source of truth for M0012.

Required revisions before acceptance:

- named type reference grammar
- qualified type name interaction with package/import rules
- nullable marker associativity and placement
- whether nullable markers may apply to function types and generic applications
- generic parameter list placement on function, struct, enum, and interface declarations
- generic parameter name grammar
- generic parameter bound grammar
- generic argument grammar
- nested generic closing delimiter policy
- function type grammar
- whether function type parameter names are allowed or deferred
- type grouping grammar
- type recovery boundaries
- explicit deferrals for unsupported type forms

## Non-Authority Finding

This review does not accept type or generic syntax. Parser implementation remains blocked until accepted source of truth resolves `docs/ambiguities/M0008-type-generic-syntax.md`.
