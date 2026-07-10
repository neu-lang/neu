# ADR-0022 main-task language review Review

Decision: request-revision-before-acceptance

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0022-declaration-syntax.md`
- Related ambiguity: `docs/ambiguities/M0008-declaration-syntax.md`
- Related milestone: `docs/milestones/M0011-declaration-parser.md`

## Findings

The proposal identifies the correct decision surface, but it does not yet contain concrete grammar that can become source of truth.

Required revisions before acceptance:

- package declaration ordering relative to imports and declarations
- import declaration aliasing and wildcard rules
- visibility modifier spellings and allowed declaration targets
- modifier ordering or a rule that modifiers are deferred
- function declaration grammar that does not depend on unresolved expression grammar
- explicit type grammar dependency for return types and parameters
- struct member grammar, or explicit deferral
- enum or sealed sum member grammar, or explicit deferral
- interface member grammar, or explicit deferral
- declaration terminator and recovery boundaries

## Non-Authority Finding

This review does not accept declaration syntax. Parser implementation remains blocked until accepted source of truth resolves `docs/ambiguities/M0008-declaration-syntax.md`.
