# Task: M0017-002 Add Nullable Type Representation

## Task Metadata

- Task ID: `M0017-002`
- Milestone: `M0017`
- Milestone File: `docs/milestones/M0017-type-representation.md`
- Status: `complete`
- Owner Agent: `Implementer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0017-002-nullable-type-representation`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
- Milestone: `docs/milestones/M0017-type-representation.md`

## Goal

Represent nullable types as explicit type records that wrap an existing non-null type identity.

## Motivation

ADR-0006 says nullable surface types are semantically explicit optional values, and M0017 requires nullable type representation to be distinguishable from non-nullable representation before M0019 can add flow typing.

## Scope

- Add a nullable type representation to the existing type model.
- Preserve the wrapped base `TypeId`.
- Add tests proving nullable and non-nullable records are distinct.
- Add a validator for the nullable representation slice.
- Update the M0017 nullable checklist item if the slice satisfies it.

## Out Of Scope

- Null literal typing.
- Smart casts and flow typing.
- Mutation invalidation.
- Nullable generic constraint semantics.
- FFI platform nullability.
- Repeated nullable marker diagnostics, because parser diagnostics already cover syntax and this task is representation only.
- Type inference, constraint solving, ownership capabilities, layout, ABI, HIR, MIR, or backend behavior.

## Required Inputs

- Milestone: `docs/milestones/M0017-type-representation.md`
- Existing type identity model from M0017-001.
- ADR-0006 nullable semantic model.
- ADR-0023 nullable surface syntax and malformed repeated marker rule.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Nullable type records preserve the wrapped base `TypeId`.
  - Nullable type records are distinguishable from their non-null base records.
  - Nested record storage remains stable when nullable records are inserted after base records.
- Negative tests:
  - The focused Rust test fails before implementation because `NullableType` and `TypeKind::Nullable` do not exist.
- Diagnostic tests:
  - Not applicable; this slice adds representation only.
- Adversarial tests:
  - Confirm this task does not introduce smart casts, null checking, FFI null mapping, inference, constraint solving, ownership capability, layout, ABI, HIR, MIR, or backend logic.

## Test-First Gate

- Test files to create before implementation:
  - `crates/newlang/tests/types.rs`
  - `docs/tests/m0017-nullable-type-representation.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `newlang::types::NullableType` and `TypeKind::Nullable` do not exist yet.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add `NullableType` as a wrapper over `TypeId`, add `TypeKind::Nullable`, and add a `TypeRecord::nullable` constructor. Do not add semantic validation or lower parsed nullable syntax in this task.

## Acceptance Criteria

- [ ] Task references exactly one milestone.
- [ ] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing change.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] Reviewer compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [x] Milestone nullable checklist is updated.

## Execution Commands

- Generate tests: `edit crates/newlang/tests/types.rs and create docs/tests/m0017-nullable-type-representation.sh`
- Verify tests fail: `cargo test -p newlang --test types`
- Ordinary tests: `cargo test -p newlang --test types && sh docs/tests/m0017-nullable-type-representation.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0017-002-nullable-type-representation.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0017-002-nullable-type-representation.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0017-nullable-type-representation.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/types.rs`
  - `docs/tests/m0017-nullable-type-representation.sh`
- Implementation files:
  - `crates/newlang/src/types.rs`
- Documentation or checklist files:
  - `docs/milestones/M0017-type-representation.md`
  - `docs/tasks/M0017-002-nullable-type-representation.md`
  - `docs/tasks/reviews/M0017-002-review.md`
  - `docs/tasks/soundness/M0017-002-soundness.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not implement nullability checking, smart casts, or flow typing.
- Do not modify parser nullable syntax or diagnostics in this task.
- Do not weaken or delete failing tests without reviewer approval.
- Do not introduce language semantics not present in `docs/SPEC.md` or `docs/adr/`.

## Ambiguities And Dependencies

- Generic nullability constraints remain unspecified and are deferred.
- Null literal typing is deferred to type checking.
- Repeated nullable marker syntax is already malformed per ADR-0023 and remains parser-owned.

## Execution Log

- 2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Task references only M0017 and scopes nullable representation without flow typing.
- 2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=`cargo test -p newlang --test types` failed because `NullableType`, `TypeKind::Nullable`, and `TypeRecord::nullable` did not exist.
- 2026-07-10 agent=Implementer phase=ordinary-tests result=pass notes=`cargo test -p newlang --test types` passed after adding nullable representation; validator initially failed only because the milestone checklist had not yet been updated.
- 2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0017-002-nullable-type-representation.md` created a passing soundness report.
- 2026-07-10 agent=Reviewer phase=review result=pass notes=`docs/scripts/review-task.sh docs/tasks/M0017-002-nullable-type-representation.md` created review and concrete review approved after source-of-truth comparison.
- 2026-07-10 agent=Build-Engineer phase=ci result=pass notes=`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, `sh docs/tests/m0017-nullable-type-representation.sh`, `sh docs/tests/m0017-type-identity-model.sh`, `sh docs/tests/m0016-name-resolution-data-model.sh`, and `sh docs/tests/m0002-workspace-ci.sh` passed.
