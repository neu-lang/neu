# Task: M0018-024 Complete Type Checking Core Milestone

## Task Metadata

- Task ID: `M0018-024`
- Milestone: `M0018`
- Milestone File: `docs/milestones/M0018-type-checking-core.md`
- Status: `complete`
- Owner main task: `main-task build check`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0018-024-complete-type-checking-core-milestone`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`

## Goal

Close M0018 after verifying the type-checking core, positive fixtures, negative diagnostics, and ambiguity blocking are complete.

## Motivation

The M0018 implementation now has accepted expression typing, declaration signatures, initializer checks, assignment checks, unresolved diagnostics, unsupported diagnostics, and a core orchestration helper. The milestone checklist still needs objective validation and closure.

## Scope

- Add a milestone completion validation script.
- Verify well-typed M0018 fixtures pass.
- Verify ill-typed M0018 fixtures diagnose.
- Verify ambiguity blockers remain recorded.
- Update the M0018 milestone completion checklist.
- Record examples skip because this task changes milestone metadata only.

## Out Of Scope

- Changing type-checking semantics.
- Changing source-language syntax.
- Changing examples.
- Implementing future M0019 or later behavior.

## Required Inputs

- `docs/milestones/M0018-type-checking-core.md`
- `docs/adr/ADR-0027-type-checking-core.md`
- `crates/compiler/src/type_check.rs`
- `crates/compiler/tests/type_check.rs`
- Existing M0018 task reports.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Completion validator requires M0018 well-typed checklist completion.
  - Completion validator requires M0018 ill-typed checklist completion.
  - Completion validator requires evidence of M0018 core orchestration tests.
- Negative tests:
  - Completion validator fails while checklist items remain open.
- Adversarial tests:
  - Closure does not weaken ADR-0027 source-of-truth checks or compiler tests.

## Test-First Gate

- Test files to edit before implementation:
  - `docs/tests/m0018-type-checking-core-complete.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - M0018 milestone checklist still has open well-typed and ill-typed items.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add the validator, prove it fails with the current open checklist, then update the milestone checklist and record completion evidence.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing change.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] main-task review compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [x] Examples update is explicitly skipped because no language-level source forms changed.

## Execution Commands

- Generate tests: create `docs/tests/m0018-type-checking-core-complete.sh`
- Verify tests fail: `sh docs/tests/m0018-type-checking-core-complete.sh`
- Ordinary tests: `sh docs/tests/m0018-type-checking-core-complete.sh && cargo test --workspace --all-targets`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0018-024-complete-type-checking-core-milestone.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0018-024-complete-type-checking-core-milestone.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0018-type-checking-core-accepted.sh && sh docs/tests/m0018-type-checking-core-complete.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0018-type-checking-core-complete.sh`
- Documentation or checklist files:
  - `docs/milestones/M0018-type-checking-core.md`
  - `docs/tasks/M0018-024-complete-type-checking-core-milestone.md`
  - `docs/tasks/reviews/M0018-024-review.md`
  - `docs/tasks/soundness/M0018-024-soundness.md`

## Forbidden Changes

- Do not change compiler implementation.
- Do not change examples.
- Do not weaken existing tests or ADR validation scripts.
- Do not mark future milestone work complete.

## Ambiguities And Dependencies

- None.

## Execution Log

- 2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0018 milestone completion task.
- 2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Created `docs/tests/m0018-type-checking-core-complete.sh` before implementation.
- 2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=`sh docs/tests/m0018-type-checking-core-complete.sh` failed before implementation because the M0018 well-typed checklist item was still open.
- 2026-07-10 main_task=Build-Engineer phase=implementation result=pass notes=Marked M0018 well-typed and ill-typed fixture checklist items complete based on existing M0018 core fixture coverage.
- 2026-07-10 main_task=Build-Engineer phase=ordinary-tests result=pass notes=`sh docs/tests/m0018-type-checking-core-complete.sh` passed and `cargo test --workspace --all-targets` passed with 171 tests.
- 2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0018-024-complete-type-checking-core-milestone.md` passed after ordinary tests.
- 2026-07-10 main_task=main-task review phase=review result=pass notes=Review approved against `docs/SPEC.md`, ADR-0027, and `docs/milestones/M0018-type-checking-core.md`.
- 2026-07-10 main_task=Examples-Curator phase=examples result=skip notes=No example update required because this task changes milestone metadata and validation only.
- 2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, `sh docs/tests/m0018-type-checking-core-accepted.sh`, `sh docs/tests/m0018-type-checking-core-complete.sh`, and `sh docs/tests/m0002-workspace-ci.sh` passed.
