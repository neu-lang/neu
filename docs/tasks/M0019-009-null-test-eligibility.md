# Task: M0019-009 Add Null-Test Eligibility

## Task Metadata

- Task ID: `M0019-009`
- Milestone: `M0019`
- Milestone File: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Status: `complete`
- Owner Agent: `Implementer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0019-009-null-test-eligibility`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
- Milestone: `docs/milestones/M0019-nullability-and-flow-typing.md`

## Goal

Filter recognized null tests to the ADR-0028 subset eligible for later refinement: immutable local bindings with known nullable wrapper types.

## Motivation

M0019 now recognizes direct null-test syntax, but smart-cast application must not proceed unless the tested expression resolves to an immutable local binding whose declared type is `T?`. This slice establishes that eligibility boundary before branch-region refinement is implemented.

## Scope

- Add an `EligibleNullTestRefinement` data record.
- Add a selector function that consumes recognized null tests, resolved names, local bindings, declaration signatures, and the type arena.
- Accept only immutable local bindings (`val`) with known nullable wrapper types.
- Preserve the original nullable type and refined non-null base type.
- Diagnose ambiguous local binding matches and mutable local refinement attempts.
- Add focused Rust tests and a docs validator.

## Out Of Scope

- Applying refinements to branch regions.
- Recording `RefinementRecord` entries.
- Recording `RefinedExpressionType` entries.
- Diagnosing nullable use sites.
- Mutation invalidation.
- Parser or name-resolution changes.
- Updating examples; this is internal analysis only.

## Required Inputs

- Null-test recognizer from M0019-008.
- Local binding model from M0016.
- Type representation from M0017.
- Type-check declaration signatures from M0018.
- Accepted ADR-0028 eligibility rules.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - A recognized null test whose name resolves to a `val` local with type `T?` becomes an eligible null-test refinement.
  - The eligible record preserves null-test node, local binding, original nullable type, and refined non-null base type.
- Negative tests:
  - Mutable `var` bindings are rejected and produce `unsupported_flow_rule(MutableLocalRefinementDeferred)`.
  - Non-nullable local bindings, unresolved names, and missing type signatures create no eligible refinement.
  - Multiple local bindings for the same resolved symbol produce `ambiguous_flow_rule(AmbiguousLocalBindingFlow)`.
- Diagnostic tests:
  - Diagnostics must point at the null-test name expression.
- Adversarial tests:
  - Eligibility must not infer types from expression text or apply refinements.

## Test-First Gate

- Test files to create before implementation:
  - `crates/newlang/tests/type_check.rs`
  - `docs/tests/m0019-null-test-eligibility.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Null-test eligibility API does not exist.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Implement a passive eligibility selector over existing side tables. Return eligible records plus a `TypeCheckReport` containing only eligibility diagnostics. Do not modify `TypeCheckReport` refinement output.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing change.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] Reviewer compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [x] Examples decision is recorded.

## Execution Commands

- Generate tests: `edit crates/newlang/tests/type_check.rs and create docs/tests/m0019-null-test-eligibility.sh`
- Verify tests fail: `cargo test -p newlang --test type_check m0019_null_test_eligibility`
- Ordinary tests: `cargo test -p newlang --test type_check m0019_null_test_eligibility && sh docs/tests/m0019-null-test-eligibility.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0019-009-null-test-eligibility.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0019-009-null-test-eligibility.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0019-null-test-eligibility.sh && sh docs/tests/m0019-null-test-recognition.sh && sh docs/tests/m0019-parser-flow-metadata.sh && sh docs/tests/m0019-flow-output-data-model.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/type_check.rs`
  - `docs/tests/m0019-null-test-eligibility.sh`
- Implementation files:
  - `crates/newlang/src/type_check.rs`
- Documentation or checklist files:
  - `docs/tasks/M0019-009-null-test-eligibility.md`
  - `docs/tasks/reviews/M0019-009-review.md`
  - `docs/tasks/soundness/M0019-009-soundness.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not apply smart casts.
- Do not add branch-region flow refinements.
- Do not diagnose nullable use sites.
- Do not change parser or name-resolution behavior.

## Ambiguities And Dependencies

- Later tasks must combine eligible null tests with if-branch region metadata before emitting `RefinementRecord` entries.

## Execution Log

- 2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created M0019 null-test eligibility task.
- 2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Added focused null-test eligibility tests and docs validator before implementation.
- 2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=`cargo test -p newlang --test type_check m0019_null_test_eligibility` failed before implementation due to unresolved eligibility imports.
- 2026-07-10 agent=Implementer phase=implement result=pass notes=Added passive eligible null-test refinement records and selector over resolution, local binding, declaration signature, and type side tables.
- 2026-07-10 agent=Test-Engineer phase=ordinary-tests result=pass notes=`cargo test -p newlang --test type_check m0019_null_test_eligibility`, `cargo test -p newlang --test type_check`, and `sh docs/tests/m0019-null-test-eligibility.sh` passed.
- 2026-07-10 agent=Adversarial-Engineer phase=adversarial-check result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0019-009-null-test-eligibility.md` passed and concrete soundness report recorded.
- 2026-07-10 agent=Reviewer phase=review result=pass notes=Compared changes against `docs/SPEC.md`, ADR-0028, and M0019; approved pending final CI.
- 2026-07-10 agent=Build-Engineer phase=ci result=pass notes=`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, and listed docs validators passed.
- 2026-07-10 agent=Implementer phase=examples-decision result=pass notes=No examples update; this task adds internal eligibility metadata and does not change source-language surface semantics.

## Handoff

- Next Agent: `Test Engineer`
- Reason: `Verify tests fail before implementation.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
  - `crates/newlang/src/type_check.rs`
