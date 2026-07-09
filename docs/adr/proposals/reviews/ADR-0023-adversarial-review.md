# ADR-0023 Adversarial Review

Decision: request-revision-before-acceptance

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0023-type-and-generic-syntax.md`
- Related ambiguity: `docs/ambiguities/M0008-type-generic-syntax.md`
- Related milestone: `docs/milestones/M0012-type-and-generic-syntax-parser.md`

## Soundness Risks

The proposal must be revised before acceptance because capability-bound syntax and generic syntax directly affect later safety analyses.

Required adversarial decisions:

- capability-bound syntax must distinguish ordinary type bounds from ownership, borrow, send, share, and coroutine-relevant capabilities
- capability-bound syntax must define whether bounds are single bounds, comma-separated lists, conjunctions, or another explicit form
- variance must be accepted explicitly or deferred explicitly
- nullable function types must not create ambiguous parse trees that affect borrow or flow-typing analysis
- generic argument parsing must not depend on expression precedence
- function type syntax must not silently introduce coroutine suspension, effect, or unsafe boundary semantics
- nested generic delimiters must be recoverable without treating lexer tokenization as semantic authority

## Attack Cases To Resolve

- `Box<T?>?` must have an accepted parse or be rejected by accepted diagnostics.
- `fun f<T: Send, Share>();` must specify whether this is one parameter with two bounds, two parameters, or invalid syntax.
- `(T) -> U?` must specify nullable return binding.
- `((T) -> U)?` must specify nullable function-type binding.

## Non-Authority Finding

This review does not accept capability-bound syntax or variance behavior. M0012 parser implementation remains blocked.
