# Task: M0021-006 Enum Subject Typing Blocker

## Task Metadata

- Task ID: `M0021-006`
- Milestone: `M0021`
- Milestone File: `docs/milestones/M0021-algebraic-data-and-exhaustiveness.md`
- Specification: `docs/SPEC.md`
- Status: `blocked`
- Owner: `Language Designer`

## Objective

Record the missing accepted semantics required to identify a `when` subject as
a bootstrap enum before implementing variant resolution or coverage checking.

## Authority Extract

- `docs/SPEC.md`, ADR-0022, ADR-0027, and ADR-0033.
- `docs/ambiguities/M0021-enum-subject-typing.md`.

## Blocker

No accepted rule specifies an enum-typed source subject, nominal enum type
resolution for an annotation or parameter, or qualified enum-variant value
expressions. Implementing those rules would invent language semantics.

## Required Resolution

An accepted ADR must specify the smallest source and type-resolution subset
that allows an M0021 `when` subject to resolve to a declared enum.

## Out Of Scope

- Inferring Kotlin behavior.
- Implementing coverage against test-only subject identities.
- Changing `docs/SPEC.md` before an ADR is accepted.

## Execution Log

- 2026-07-11 agent=Main phase=blocker-recorded result=blocked evidence=ADR-0033 subject rule lacks an accepted source-to-enum typing path. handoff=Language-Designer
