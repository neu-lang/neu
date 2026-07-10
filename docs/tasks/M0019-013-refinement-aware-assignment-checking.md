# Task: M0019-013 Add Refinement-Aware Assignment Checking

## Task Metadata

- Task ID: `M0019-013`
- Milestone: `M0019`
- Milestone File: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0019-013-refinement-aware-assignment-checking`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
- Milestone: `docs/milestones/M0019-nullability-and-flow-typing.md`

## Goal

Check assignment statements using valid per-use refinements and diagnose unrefined `T?` values assigned to `T` with the accepted M0019 diagnostic.

## Motivation

M0019-012 records non-null per-use type views, but assignment compatibility does not consume them. ADR-0028 requires a direct refined `T` use to satisfy a `T` target while an unrefined `T?` use reports `invalid_nullable_use` rather than a generic mismatch.

## Scope

- Add an M0019 assignment-statement checker alongside the unchanged M0018 checker.
- Use a `RefinedExpressionType` only for the exact assigned value expression.
- Validate that the refined record's original type matches the value's original nullable type and that its refined type is that nullable wrapper's base.
- Require a unique matching refinement-region record, exact resolved local binding identity, and AST containment inside that branch before trusting a per-use view.
- Record successful assignment compatibility with the effective refined value type.
- Report `InvalidNullableUse` with `NullableAssignmentWithoutRefinement` for unrefined `T? -> T` assignments.
- Preserve ADR-0027 exact, `Null -> T?`, and `T -> T?` compatibility.
- Preserve ordinary `TypeMismatch` for `Null -> T` and unrelated nullable mismatches.
- Add focused Rust tests and a docs validator.

## Out Of Scope

- Local declaration initializer checking.
- Grouped-expression propagation of refined views.
- Nullable name use in other expected-type contexts.
- Mutation invalidation.
- Flow-pass orchestration.
- Parser, resolution, type representation, or diagnostic rendering changes.

## Required Inputs

- M0018 assignment compatibility rules from ADR-0027.
- Per-use `RefinedExpressionType` records from M0019-012.
- Refinement-region records, exact local binding resolutions, and AST region metadata from M0019-010 through M0019-012.
- ADR-0028 nullable assignment and diagnostic rules.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - A valid refined `T?` value use assigned to `T` succeeds and records `T` as the effective value type.
  - Exact assignment and ADR-0027 nullable-target exceptions remain accepted.
- Negative tests:
  - An unrefined `T?` value assigned to its base `T` produces no successful assignment check.
  - `Null -> T` and unrelated `U? -> T` remain ordinary type mismatches.
- Diagnostic tests:
  - Unrefined `T? -> T` reports `InvalidNullableUse`, `NullableAssignmentWithoutRefinement`, the assigned value node, expected `T`, and actual `T?`.
- Adversarial tests:
  - Refinement records with a mismatched original nullable type or a refined type that is not the wrapper base are ignored.
  - Duplicate refined views and forged out-of-region views are ignored.
  - The real region-aware producer composes with assignment checking so inside-branch use succeeds and after-branch use fails.

## Test-First Gate

- Test files to create before implementation:
  - `crates/compiler/tests/type_check.rs`
  - `docs/tests/m0019-refinement-aware-assignment-checking.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - M0019 refinement-aware assignment checking API does not exist.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Reuse the existing assignment compatibility predicate through a shared internal assignment loop. Keep the M0018 public function behavior unchanged and add an M0019 entry point that validates complete flow provenance before classifying diagnostics.

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

- Generate tests: `edit crates/compiler/tests/type_check.rs and create docs/tests/m0019-refinement-aware-assignment-checking.sh`
- Verify tests fail: `cargo test -p compiler --test type_check m0019_refinement_aware_assignment`
- Ordinary tests: `cargo test -p compiler --test type_check m0019_refinement_aware_assignment && sh docs/tests/m0019-refinement-aware-assignment-checking.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0019-013-refinement-aware-assignment-checking.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0019-013-refinement-aware-assignment-checking.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0019-refinement-aware-assignment-checking.sh && sh docs/tests/m0019-refined-expression-type-records.sh && sh docs/tests/m0019-local-binding-resolution-identity.sh && sh docs/tests/m0019-branch-refinement-records.sh && sh docs/tests/m0019-null-test-eligibility.sh && sh docs/tests/m0019-null-test-recognition.sh && sh docs/tests/m0019-parser-flow-metadata.sh && sh docs/tests/m0019-flow-output-data-model.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/compiler/tests/type_check.rs`
  - `docs/tests/m0019-refinement-aware-assignment-checking.sh`
- Implementation files:
  - `crates/compiler/src/type_check.rs`
- Documentation or checklist files:
  - `docs/tasks/M0019-013-refinement-aware-assignment-checking.md`
  - `docs/tasks/reviews/M0019-013-review.md`
  - `docs/tasks/soundness/M0019-013-soundness.md`
- Example files, if the compiler-supported surface changes:
  - `examples/current/accepted_nullability_flow.neu`
  - `examples/current/README.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md` or `docs/adr/`.
- Do not change the public behavior of `type_assignment_statements`.
- Do not trust inconsistent refined side-table entries.
- Do not classify `Null -> T` or unrelated `U? -> T` as nullable-use diagnostics.
- Do not implement local initializer checking, grouped refinement propagation, invalidation, or orchestration.

## Ambiguities And Dependencies

- A later M0019 task must apply the same accepted nullable-use rule to local declaration initializers, including the accepted example's `val definite: String = maybe` form.
- Grouped refined values require propagation or innermost-use tracking in a later task.

## Execution Log

- 2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0019 assignment-statement nullable-use integration task.
- 2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Added refined success, nullable-use diagnostic boundary, M0018 compatibility, ordinary mismatch, inconsistent-view tests, and a docs validator before implementation.
- 2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=Focused tests failed on the missing `type_m0019_assignment_statements` import and the validator failed on the absent API.
- 2026-07-10 main_task=main-task implementation phase=implement result=pass notes=Added M0019 assignment checking through the shared M0018 compatibility loop with validated per-use views and nullable-specific diagnostic classification.
- 2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=Five focused tests, all 61 type-check tests, docs validator, and `git diff --check` passed; `cargo fmt --all` applied mechanical formatting.
- 2026-07-10 main_task=Adversarial-Engineer phase=add-attacks result=pass notes=Added duplicate refined-view attack after ordinary tests to prevent insertion-order selection.
- 2026-07-10 main_task=Adversarial-Engineer phase=adversarial-check result=pass notes=All six focused tests and the harness passed; concrete soundness report covers inconsistent, duplicate, and misclassified assignment attacks.
- 2026-07-10 main_task=main-task review phase=review result=pass notes=Compared implementation against SPEC, ADR-0027, ADR-0028, and M0019; no findings and M0018 public behavior remains unchanged; approved pending final CI.
- 2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=Formatting, workspace clippy with warnings denied, all 201 workspace tests, all listed M0019 validators, and the M0002 baseline CI gate passed.
- 2026-07-10 main_task=Task-Decomposer phase=milestone-checklist result=pass notes=No M0019 completion item changed; assignment statements now consume refinements, but local initializers, orchestration, and invalidation remain incomplete.
- 2026-07-10 main_task=main-task implementation phase=examples-decision result=pass notes=Updated the current nullability example with direct assignment from a refined nullable local and documented the exact implemented versus pending M0019 surface.
- 2026-07-10 main_task=main-task review phase=precommit-audit result=fail notes=Cached diff revealed the shared loop changed M0018 `T? -> T` diagnostics; required explicit M0018/M0019 mode separation.
- 2026-07-10 main_task=main-task implementation phase=review-fix result=pass notes=Added explicit private diagnostic mode and an M0018 nullable-to-base regression test; full validation must be rerun.
- 2026-07-10 main_task=Adversarial-Engineer phase=parallel-review result=fail notes=Independent GPT-5.6 Sol review found that assignment checking trusted type-consistent refined records without validating branch or binding provenance.
- 2026-07-10 main_task=main-task implementation phase=provenance-fix result=pass notes=Required unique flow record, refinement-region record, exact resolved binding, AST branch containment, and consistent nullable/base types before consuming a refined view.
- 2026-07-10 main_task=main-task test work phase=post-review-regression result=pass notes=Seven focused assignment tests, the M0018 compatibility regression, and all 64 type-check tests passed after provenance validation.
- 2026-07-10 main_task=Build-Engineer phase=final-ci-rerun result=pass notes=Formatting, workspace clippy with warnings denied, all 203 workspace tests, all listed M0019 validators, and the M0002 baseline CI gate passed after both review fixes.

## Handoff

- Next main task: `main-task task planning`
- Reason: `Create the next M0019 task for refinement-aware local declaration initializers.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
  - `crates/compiler/src/type_check.rs`
