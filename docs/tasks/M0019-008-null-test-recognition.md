# Task: M0019-008 Add Null-Test Recognition

## Task Metadata

- Task ID: `M0019-008`
- Milestone: `M0019`
- Milestone File: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0019-008-null-test-recognition`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
- Milestone: `docs/milestones/M0019-nullability-and-flow-typing.md`

## Goal

Recognize ADR-0028 direct null-test shapes from parser metadata without applying refinements.

## Motivation

M0019 flow typing needs a flow-specific condition recognizer for `x != null`, `null != x`, `x == null`, and `null == x`. Parser flow metadata now exposes binary operands and operators, so the next step is to identify recognized null-test conditions and their refined branch direction.

## Scope

- Add a `RecognizedNullTest` data record.
- Add a refined branch enum for then-branch versus else-branch refinement.
- Add a recognition function that consumes parsed binary expression and literal metadata.
- Recognize only the four direct ADR-0028 shapes.
- Add focused Rust tests and a docs validator.

## Out Of Scope

- Checking immutable local binding eligibility.
- Resolving names.
- Applying smart casts or recording refinements.
- Diagnosing nullable misuse.
- Producing unsupported-flow diagnostics for rejected conditions.
- Changing parser metadata.
- Updating examples; this is internal analysis only.

## Required Inputs

- Parser binary expression metadata from M0019-007.
- Parser literal metadata from M0018.
- Accepted ADR-0028 null-test recognition rules.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Recognizes `x != null` and `null != x` as then-branch refinements.
  - Recognizes `x == null` and `null == x` as else-branch refinements.
  - Records binary expression node, name expression node, null literal node, operator, and refined branch.
- Negative tests:
  - Ignores non-equality operators, non-null literals, null-null comparisons, and name-name comparisons.
  - Does not apply refinements or record refined expression types.
- Diagnostic tests:
  - Not applicable; unsupported diagnostics are later behavior.
- Adversarial tests:
  - Recognizer must not type check binary expressions or rely on overload/equality semantics.

## Test-First Gate

- Test files to create before implementation:
  - `crates/newlang/tests/type_check.rs`
  - `docs/tests/m0019-null-test-recognition.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Null-test recognizer API does not exist.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Implement a passive recognizer over `ParsedBinaryExpression` and `ParsedLiteralExpression` side tables. Return recognized null-test records only; do not modify `TypeCheckReport`.

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

- Generate tests: `edit crates/newlang/tests/type_check.rs and create docs/tests/m0019-null-test-recognition.sh`
- Verify tests fail: `cargo test -p newlang --test type_check m0019_null_test`
- Ordinary tests: `cargo test -p newlang --test type_check m0019_null_test && sh docs/tests/m0019-null-test-recognition.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0019-008-null-test-recognition.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0019-008-null-test-recognition.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0019-null-test-recognition.sh && sh docs/tests/m0019-parser-flow-metadata.sh && sh docs/tests/m0019-flow-output-data-model.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/type_check.rs`
  - `docs/tests/m0019-null-test-recognition.sh`
- Implementation files:
  - `crates/newlang/src/type_check.rs`
- Documentation or checklist files:
  - `docs/tasks/M0019-008-null-test-recognition.md`
  - `docs/tasks/reviews/M0019-008-review.md`
  - `docs/tasks/soundness/M0019-008-soundness.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not apply smart casts.
- Do not add flow refinements.
- Do not diagnose nullable misuse.
- Do not type check binary expressions.

## Ambiguities And Dependencies

- Later tasks must combine recognized null tests with local binding eligibility and branch region metadata.

## Execution Log

- 2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0019 null-test recognition task.
- 2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Added focused null-test recognition tests and docs validator before implementation.
- 2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=`cargo test -p newlang --test type_check m0019_null_test` failed before implementation due to unresolved recognizer imports.
- 2026-07-10 main_task=main-task implementation phase=implement result=pass notes=Added passive null-test records and recognizer over parser binary/literal metadata only.
- 2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=`cargo test -p newlang --test type_check m0019_null_test`, `cargo test -p newlang --test type_check`, and `sh docs/tests/m0019-null-test-recognition.sh` passed.
- 2026-07-10 main_task=Adversarial-Engineer phase=adversarial-check result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0019-008-null-test-recognition.md` passed and concrete soundness report recorded.
- 2026-07-10 main_task=main-task review phase=review result=pass notes=Compared changes against `docs/SPEC.md`, ADR-0028, and M0019; approved pending final CI.
- 2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, and listed docs validators passed.
- 2026-07-10 main_task=main-task implementation phase=examples-decision result=pass notes=No examples update; this task adds internal recognizer metadata and does not change source-language surface semantics.

## Handoff

- Next main task: `main-task test work`
- Reason: `Verify tests fail before implementation.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
  - `crates/newlang/src/type_check.rs`
