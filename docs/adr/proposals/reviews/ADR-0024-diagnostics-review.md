# ADR-0024 Diagnostics Review

Decision: request-revision-before-acceptance

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0024-expression-statement-pattern-syntax.md`
- Related ambiguity: `docs/ambiguities/M0008-expression-statement-pattern-syntax.md`
- Related milestone: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`

## Diagnostic Findings

The proposal lists candidate diagnostic topics, but it does not yet define diagnostic categories with primary span, recovery action, source-of-truth citation, and safe suggestion policy.

Expression diagnostics need recovery boundaries that preserve parser progress without hiding statement or block boundaries. The accepted grammar must decide whether recovery synchronizes at comma, semicolon, right parenthesis, right brace, arm arrows, declaration keywords, or end of file.

Statement diagnostics need clear boundary rules. Missing statement, unsupported statement form, malformed assignment, malformed return statement, and malformed variable declaration must recover without consuming the next valid declaration or block close.

Pattern diagnostics need separate handling from expression diagnostics. Malformed pattern recovery must not silently reinterpret a pattern as an expression or vice versa.

Unsafe and coroutine diagnostics must either be included with concrete primary span and recovery action or explicitly deferred.

Safe suggestion policy must be conservative. Suggestions must not invent missing expression semantics or recommend syntax that is not accepted.

## Required Revisions

- Define diagnostic names exactly.
- Define primary span for every diagnostic.
- Define recovery action for expression, statement, block, pattern, unsafe, and coroutine errors.
- Define safe suggestion policy for every diagnostic.
- Require all accepted body syntax diagnostics to cite ADR-0015 and ADR-0024.
- Include malformed delimiter and missing arm-body cases if match or `when` syntax is accepted.

## Non-Authority Reminder

This review does not accept ADR-0024. M0013 parser implementation remains blocked.
