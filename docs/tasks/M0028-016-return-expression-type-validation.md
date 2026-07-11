# Task: M0028-016 Return Expression Type Validation

## Task Metadata

- Task ID: `M0028-016`
- Milestone: `M0028`
- Milestone File: `docs/milestones/M0028-executable-expression-frontend-completion.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Validate explicit bootstrap return expressions against their function's declared
`Int` return type.

## Authority Extract

- ADR-0041 explicit return and `return_type_mismatch` requirements.
- ADR-0042 executable `Int` subset.

## Resolved Dependency

ADR-0054 defines diagnostic provenance and recovery. Implementation remains
the next task action.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=blocked evidence=return_type_mismatch diagnostic contract is incomplete. handoff=semantic-design
- 2026-07-11 main_task=main phase=semantic-resolution result=pass evidence=ADR-0054 accepted after required reviews. handoff=test
