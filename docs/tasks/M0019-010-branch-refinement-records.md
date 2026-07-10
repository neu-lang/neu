# Task: M0019-010 Add Branch Refinement Records

## Task Metadata

- Task ID: `M0019-010`
- Milestone: `M0019`
- Milestone File: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0019-010-branch-refinement-records`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
- Milestone: `docs/milestones/M0019-nullability-and-flow-typing.md`

## Goal

Record branch-scoped `RefinementRecord` entries for eligible null tests in matching `if` branches.

## Motivation

M0019 now recognizes direct null tests and filters them to immutable nullable local bindings. The next step is to attach those eligible refinements to the syntactic branch regions defined by ADR-0028 without yet applying refined per-use expression types.

## Scope

- Add a function that consumes `EligibleNullTestRefinement` records and parser `ParsedIfExpression` metadata.
- Record a `RefinementRecord` for `!= null` tests in the then branch block.
- Record a `RefinementRecord` for `== null` tests in the else branch block only when an else branch exists.
- Preserve binding use node, originating null-test expression node, local binding identity, original nullable type, and refined non-null type.
- Add focused Rust tests and a docs validator.

## Out Of Scope

- Walking branch contents.
- Recording `RefinedExpressionType` entries.
- Diagnosing nullable use sites.
- Mutation invalidation.
- Nested scope or shadowing behavior.
- Parser, name-resolution, or type-representation changes.
- Updating examples; this is internal analysis output only.

## Required Inputs

- Parser `ParsedIfExpression` metadata from M0019-007.
- `EligibleNullTestRefinement` from M0019-009.
- Accepted ADR-0028 branch region rules.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Then-branch eligible tests record a refinement for the then block.
  - Else-branch eligible tests record a refinement for the else block.
  - Refinement records preserve the expected binding use, originating null test, local binding, original nullable type, and refined non-null type.
- Negative tests:
  - Else-branch refinements are not recorded when no else branch exists.
  - Eligible tests whose null-test expression is not an `if` condition produce no refinement.
  - This slice does not record refined expression type entries.
- Diagnostic tests:
  - None; unsupported nullable-use and invalidation diagnostics are later behavior.
- Adversarial tests:
  - Branch attachment must depend on parser `if` condition node identity, not source text or operator text.

## Test-First Gate

- Test files to create before implementation:
  - `crates/compiler/tests/type_check.rs`
  - `docs/tests/m0019-branch-refinement-records.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Branch refinement API does not exist.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Implement a passive function that maps eligible null-test direction to the corresponding parser branch block and records `RefinementRecord` entries in a `TypeCheckReport`. Do not walk branch contents or create per-use refined types.

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
- [x] Examples decision is recorded.

## Execution Commands

- Generate tests: `edit crates/compiler/tests/type_check.rs and create docs/tests/m0019-branch-refinement-records.sh`
- Verify tests fail: `cargo test -p compiler --test type_check m0019_branch_refinement`
- Ordinary tests: `cargo test -p compiler --test type_check m0019_branch_refinement && sh docs/tests/m0019-branch-refinement-records.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0019-010-branch-refinement-records.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0019-010-branch-refinement-records.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0019-branch-refinement-records.sh && sh docs/tests/m0019-null-test-eligibility.sh && sh docs/tests/m0019-null-test-recognition.sh && sh docs/tests/m0019-parser-flow-metadata.sh && sh docs/tests/m0019-flow-output-data-model.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/compiler/tests/type_check.rs`
  - `docs/tests/m0019-branch-refinement-records.sh`
- Implementation files:
  - `crates/compiler/src/type_check.rs`
- Documentation or checklist files:
  - `docs/tasks/M0019-010-branch-refinement-records.md`
  - `docs/tasks/reviews/M0019-010-review.md`
  - `docs/tasks/soundness/M0019-010-soundness.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not record refined expression type entries.
- Do not diagnose nullable use sites.
- Do not implement mutation invalidation.
- Do not change parser or name-resolution behavior.

## Ambiguities And Dependencies

- Later tasks must walk branch-region contents and record per-use refined expression types.

## Execution Log

- 2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0019 branch refinement record task.
- 2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Added focused branch refinement record tests and docs validator before implementation.
- 2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=`cargo test -p compiler --test type_check m0019_branch_refinement` failed before implementation due to unresolved branch refinement import.
- 2026-07-10 main_task=main-task implementation phase=implement result=pass notes=Added passive branch refinement recorder from eligible null-test records and parser if-expression metadata.
- 2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=`cargo test -p compiler --test type_check m0019_branch_refinement`, `cargo test -p compiler --test type_check`, and `sh docs/tests/m0019-branch-refinement-records.sh` passed.
- 2026-07-10 main_task=Adversarial-Engineer phase=adversarial-check result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0019-010-branch-refinement-records.md` passed and concrete soundness report recorded.
- 2026-07-10 main_task=main-task review phase=review result=pass notes=Compared changes against `docs/SPEC.md`, ADR-0028, and M0019; approved pending final CI.
- 2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, and listed docs validators passed.
- 2026-07-10 main_task=main-task implementation phase=examples-decision result=pass notes=No examples update; this task adds internal branch refinement records and does not change source-language surface semantics.

## Handoff

- Next main task: `main-task test work`
- Reason: `Verify tests fail before implementation.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
  - `crates/compiler/src/type_check.rs`
