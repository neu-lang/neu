# ADR-0024 Simplicity Review

Decision: request-revision-before-acceptance

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0024-expression-statement-pattern-syntax.md`
- Related ambiguity: `docs/ambiguities/M0008-expression-statement-pattern-syntax.md`
- Related milestone: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`

## Simplicity Findings

The recommended small Kotlin-like custom body grammar is the right direction, but the proposal is still too broad to accept because it lists many possible constructs without choosing a bootstrap subset.

The accepted M0013 grammar should prefer a minimal core: literals, names, grouping, calls if needed, member access if needed, a small precedence table, blocks, return statements, variable declarations if required, simple assignments, `if`, and a small pattern subset only if match syntax is included.

Coroutine syntax should be deferred unless the project has an immediate parser need. Unsafe block syntax should be either a minimal explicit form or deferred, but not left implicit.

Advanced pattern forms, destructuring, lambdas, labels, ranges, annotations, and error-handling sugar should remain deferred unless a later milestone proves they are needed.

## Required Revisions

- Pick a small bootstrap grammar instead of a menu of possible constructs.
- Mark each excluded construct as explicitly deferred.
- Avoid accepting both `when` and match-style syntax unless a strong reason exists.
- Avoid expression-as-everything complexity unless block result rules are defined.
- Keep coroutine and unsafe syntax separate from ordinary expression grammar unless accepted explicitly.

## Non-Authority Reminder

This review does not accept ADR-0024. M0013 parser implementation remains blocked.
