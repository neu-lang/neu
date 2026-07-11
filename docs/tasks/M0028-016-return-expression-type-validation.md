# Task: M0028-016 Return Expression Type Validation

## Task Metadata

- Task ID: `M0028-016`
- Milestone: `M0028`
- Milestone File: `docs/milestones/M0028-executable-expression-frontend-completion.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

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
- 2026-07-11 main_task=main phase=test-first result=fail evidence=return type report API was absent. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=known incompatible explicit returns report on the value expression; unresolved values do not cascade. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets; docs/tests/m0028-return-expression-type-validation.sh. handoff=adversarial
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=unresolved return expressions do not cascade and mismatched returns produce no typed fact; docs/tasks/soundness/M0028-016-soundness.md. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0041 and ADR-0054 compliance confirmed; docs/tasks/reviews/M0028-016-review.md. handoff=commit
- 2026-07-11 main_task=main phase=ci result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets; docs/tests/m0028-return-expression-type-validation.sh. handoff=commit
