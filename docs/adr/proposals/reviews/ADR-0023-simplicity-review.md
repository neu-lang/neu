# ADR-0023 Simplicity Review

Decision: request-revision-before-acceptance

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0023-type-and-generic-syntax.md`
- Related ambiguity: `docs/ambiguities/M0008-type-generic-syntax.md`
- Related milestone: `docs/milestones/M0012-type-and-generic-syntax-parser.md`

## Findings

The small Kotlin-like custom type grammar direction is appropriate, but acceptance requires a smaller concrete grammar and explicit deferrals.

Required simplifications before acceptance:

- accept named type references before advanced type forms
- accept nullable syntax only after binding rules are explicit
- accept generic parameters and arguments only if delimiter and recovery rules are explicit
- defer variance unless a near-term milestone requires it
- defer wildcard or star-projection types
- defer receiver function types
- defer associated, higher-kinded, dependent, intersection, and union types
- defer coroutine suspension markers in function types
- avoid importing Kotlin type-system edge cases

## Simplicity Position

The accepted bootstrap grammar should be the smallest grammar that unblocks M0012 fixtures, type AST nodes, and later type representation. Features needed only by future type-system milestones should be explicitly deferred.

## Non-Authority Finding

This review does not accept the proposal. It requests a concrete minimal grammar before Chief Architect approval.
