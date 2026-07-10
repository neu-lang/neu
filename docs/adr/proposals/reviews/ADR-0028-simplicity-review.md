# ADR-0028 Simplicity Review

## Metadata

- Proposal: `ADR-0028`
- Milestone: `M0019`
- main-task review: `main-task simplicity check`
- Date: `2026-07-10`
- Decision: `approve direction, request revision before acceptance`

## Inputs Read

- `docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md`
- `docs/milestones/M0019-nullability-and-flow-typing.md`
- `docs/ambiguities/M0019-nullability-and-flow-typing.md`

## Simplicity Assessment

The proposal chooses a narrow subset: local immutable null refinements only, with members, calls, generics, exclusive borrows, aliases, patterns, unsafe, FFI, and coroutine suspension deferred.

This is the right shape for M0019 because it avoids building a general control-flow and effect system before accepted semantics require one.

## Required Simplifications Or Clarifications

- Keep boolean combination refinement deferred for the first accepted version.
- Keep exclusive-borrow refinements deferred until borrow checker milestones.
- Keep member and property stability rules deferred.
- Avoid adding a general CFG abstraction unless the accepted branch region boundaries require it.
- Define the initial implementation as a narrow subset rather than a framework for all future flow typing.

## Decision

Approve the narrow subset direction, but request revision before acceptance so the final ADR is concrete enough for implementation without adding unnecessary abstraction.
