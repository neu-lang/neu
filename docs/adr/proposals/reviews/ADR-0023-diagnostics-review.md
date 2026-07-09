# ADR-0023 Diagnostics Review

Decision: request-revision-before-acceptance

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0023-type-and-generic-syntax.md`
- Diagnostic authority: `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- Related milestone: `docs/milestones/M0012-type-and-generic-syntax-parser.md`

## Required Diagnostic Decisions

The accepted type syntax must define type syntax diagnostics before parser implementation. ADR-0015 requires syntax diagnostics to be part of the language contract once grammar is accepted.

Required type syntax diagnostics include:

- missing type name
- malformed nullable type
- malformed generic parameter list
- malformed generic argument list
- missing generic bound
- malformed capability bound
- malformed function type
- unsupported type form
- unexpected token in type syntax

Each diagnostic must define:

- primary span
- recovery action
- source-of-truth citation
- whether secondary spans are required
- whether a safe suggestion is permitted

## Recovery Requirements

The accepted syntax must define recovery boundaries for:

- type annotations in declaration headers
- generic parameter lists in declaration headers
- generic argument lists in type references
- function type parameter lists
- capability-bound lists

## Non-Authority Finding

The draft proposal identifies diagnostic categories, but no type syntax diagnostics are accepted yet.
