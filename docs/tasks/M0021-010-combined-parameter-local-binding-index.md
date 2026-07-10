# Task: M0021-010 Combined Parameter And Local Binding Index

## Task Metadata

- Task ID: `M0021-010`
- Milestone: `M0021`
- Milestone File: `docs/milestones/M0021-algebraic-data-and-exhaustiveness.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `Test Engineer`, then `Implementer`

## Objective

Build one scoped local-binding index containing both ADR-0034 parameters and
existing local declarations, enabling ordinary name resolution to bind a
parameter subject.

## Scope

- Combine parameter and local binding records in lexical scope order.

## Out Of Scope

- Enum type resolution, `when` diagnostics, variant resolution, and coverage.

## Required Tests Before Implementation

- A bare use of a parameter resolves to that parameter binding.

## Acceptance Criteria

- [x] Tests fail before the combined index API exists.
- [x] Existing local binding semantics remain unchanged.

## Execution Log

- 2026-07-11 agent=Main phase=create-task result=pass evidence=ordinary name-binding prerequisite only. handoff=Test-Engineer
- 2026-07-11 agent=Main phase=test-first result=fail evidence=combined scoped binding API was absent. handoff=Implementer
- 2026-07-11 agent=Main phase=implementation result=pass evidence=ordinary local resolver can bind parameter uses through combined lexical bindings. handoff=Reviewer
- 2026-07-11 agent=Main phase=ordinary-tests result=pass evidence=focused resolution test, formatter, strict clippy, and workspace tests passed. handoff=Adversarial-Engineer
- 2026-07-11 agent=Main phase=adversarial-check result=pass evidence=combined index adds no type or match semantics. handoff=Reviewer
- 2026-07-11 agent=Main phase=review result=approve evidence=scope and full validation verified. handoff=none
