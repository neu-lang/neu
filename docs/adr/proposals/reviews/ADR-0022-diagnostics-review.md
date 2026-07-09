# ADR-0022 Diagnostics Review

Decision: request-revision-before-acceptance

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0022-declaration-syntax.md`
- Diagnostic authority: `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- Related milestone: `docs/milestones/M0011-declaration-parser.md`

## Required Diagnostic Decisions

The accepted declaration syntax must define declaration diagnostics before parser implementation. ADR-0015 requires syntax diagnostics to be part of the language contract once a grammar is accepted.

Required declaration diagnostics include:

- misplaced package declaration
- misplaced import declaration
- unsupported declaration modifier
- missing declaration name
- malformed declaration header
- invalid member declaration position
- unexpected token in declaration body

Each diagnostic must define:

- primary span
- recovery action
- source-of-truth citation
- whether secondary spans are required
- whether a safe suggestion is permitted

## Non-Authority Finding

The draft proposal identifies diagnostic consequences, but no declaration diagnostics are accepted yet.
