# ADR-0027 Adversarial Review

## Metadata

- ADR: `ADR-0027`
- Milestone: `M0018`
- main-task review: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `request revision before acceptance`

## Soundness Concerns

This review focuses on soundness risks in the proposed M0018 bootstrap subset.

The proposal must not allow guessed primitive categories to leak into ownership, borrow, or backend phases. If `Int`, `Bool`, `String`, `Unit`, and `Null` are accepted for M0018, the accepted text must state they are type-checking identities only unless later ABI rules say otherwise.

Nullable assignment is a safety-sensitive rule. The accepted ADR must decide whether a non-null base can be assigned to a nullable wrapper and must preserve the invariant that `null` never inhabits non-nullable types.

Call resolution must not accidentally introduce overload resolution, generic instantiation, member lookup, constructor lookup, or type-directed lookup. The proposal should require single-candidate evidence from accepted name resolution.

Function type application should not include coroutine suspension, receiver function types, effects, unsafe boundaries, or ownership behavior.

## Required Revisions

- State that primitive bootstrap categories have no ABI meaning.
- State exact nullable assignment rules.
- Require single-candidate direct function call evidence or defer calls.
- Keep function type application structural and non-suspending, or defer it.
- Add explicit "must not implement" text for overloads, conversions, member calls, and generic solving.

## Boundary

This review is not accepted source of truth. Do not implement M0018 type checking from this proposal or this review.
