# ADR-0024 Adversarial Review

Decision: request-revision-before-acceptance

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0024-expression-statement-pattern-syntax.md`
- Related ambiguity: `docs/ambiguities/M0008-expression-statement-pattern-syntax.md`
- Related milestone: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`

## Soundness Risks

Ownership scope is the highest-risk unresolved area. Block grammar and statement boundaries decide when values are destroyed, when moves occur, and where borrows end. The proposal must make ownership scope visible enough for later analysis.

Pattern syntax can create bindings, moves, borrows, and smart-cast refinements. Without concrete pattern grammar and binding rules, parser fixtures could accidentally commit to unsound destructuring or aliasing behavior.

Coroutine syntax must not be inferred from keywords or Kotlin conventions. Suspension points affect borrow validity and structured concurrency guarantees. If coroutine syntax is not in M0013, the accepted ADR must explicitly defer it.

Unsafe block syntax must be explicit or explicitly deferred. A vague unsafe construct risks weakening safe-code guarantees and module audit boundaries from ADR-0018.

Control-flow constructs must distinguish syntax from semantics. Parsing `return`, loops, conditionals, or match arms must not imply type checking, exhaustiveness, move analysis, or coroutine analysis.

## Attack Cases Required Before Acceptance

- A block that moves a value and then tries to use it later.
- A pattern that binds by value versus by reference, if either syntax is accepted.
- A conditional that creates a smart-cast refinement and then mutates the subject.
- A coroutine-like construct containing a borrow across a possible suspension point.
- An unsafe block nested inside ordinary safe code.
- A match or `when` arm with missing body syntax.

## Required Revisions

- Define ownership scope boundaries syntactically.
- Define pattern binding positions and explicitly defer binding modes if unresolved.
- Include or defer coroutine syntax.
- Include or defer unsafe block syntax.
- Ensure every accepted grammar form can be parsed without assuming type checking or flow analysis.

## Non-Authority Reminder

This review does not accept ADR-0024. M0013 parser implementation remains blocked.
