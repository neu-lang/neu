# ADR-0024 main-task language review Review

Decision: request-revision-before-acceptance

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0024-expression-statement-pattern-syntax.md`
- Related ambiguity: `docs/ambiguities/M0008-expression-statement-pattern-syntax.md`
- Related milestone: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`

## Findings

The proposal correctly identifies that accepted ADRs define semantics but not concrete grammar. It is not yet acceptable as source of truth because it does not define concrete grammar for expression, statement, block, or pattern syntax.

Acceptance requires a complete operator precedence and associativity table. Token spellings from ADR-0021 are not sufficient authority for expression parsing.

Pattern grammar must be concrete before parser fixtures can exist. At minimum, review must decide wildcard patterns, literal patterns, identifier binding patterns, qualified enum-case patterns, grouped patterns, and any destructuring forms included in M0013.

Statement grammar must define block boundaries, statement separators, optional semicolon behavior, declaration statements, return statements, assignment syntax, and how expression statements are recognized.

The proposal must explicitly state whether blocks are expressions, statements, or both, because that decision affects ownership scope, destruction points, type checking, and control-flow analysis.

## Required Revisions

- Add concrete expression grammar entry points.
- Add operator precedence and associativity.
- Add concrete statement grammar.
- Add concrete block grammar.
- Add concrete pattern grammar.
- Decide whether unsafe and coroutine syntax are included or deferred.
- Add recovery boundaries for expressions, statements, blocks, and patterns.
- Add accepted diagnostic categories with primary span, recovery action, and safe suggestion policy.

## Non-Authority Reminder

This review does not accept ADR-0024. M0013 parser implementation remains blocked.
