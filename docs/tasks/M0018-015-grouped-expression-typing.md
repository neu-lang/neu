# Task: M0018-015 Grouped Expression Typing

## Task Metadata

- Task ID: `M0018-015`
- Milestone: `M0018`
- Milestone File: `docs/milestones/M0018-type-checking-core.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0018-015-grouped-expression-typing`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`

## Goal

Type grouped expressions by propagating the already-known inner expression type to the grouped expression node.

## Motivation

ADR-0027 includes grouped expressions in M0018 and states they type to the inner expression type. M0018-014 exposed parser metadata connecting grouped expression nodes to their inner expression nodes.

## Scope

- Consume parser grouped expression metadata from M0018-014.
- Consume existing expression type side-table entries.
- Emit expression type side-table entries for grouped expressions whose inner expression already has a type.
- Preserve grouped expression metadata order.
- Skip grouped expressions whose inner expression has no known type.

## Out Of Scope

- Typing the inner expression itself.
- Merging type-check reports.
- Inferring missing expression types.
- Typing unsupported expressions.
- Changing parser metadata.
- Assignment checks, call typing, member lookup, ownership, borrow, HIR, MIR, or backend behavior.

## Required Inputs

- `ParsedGroupedExpression` records.
- Existing `ExpressionType` records.
- ADR-0027 grouped expression typing rule.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - A grouped expression with a typed inner expression receives the same type.
  - Multiple grouped expressions preserve grouped metadata order.
  - Nested grouped expressions can be typed when the inner grouped expression type is already present.
- Negative tests:
  - A grouped expression whose inner expression has no type is skipped.
  - No declaration signatures, assignment checks, or diagnostics are synthesized.
- Adversarial tests:
  - Grouped expression typing does not infer literals, resolve names, type calls, or implement unsupported expression rules.

## Test-First Gate

- Test files to edit before implementation:
  - `crates/newlang/tests/type_check.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Grouped expression typing entry point does not exist yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a pure helper that scans grouped expression metadata in order, finds the inner expression type in a supplied expression type slice, and emits grouped expression `ExpressionType` records for matches.

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
- [x] No compiler behavior beyond grouped expression type propagation is introduced.

## Execution Commands

- Generate tests: edit `crates/newlang/tests/type_check.rs`
- Verify tests fail: `cargo test --workspace --all-targets`
- Ordinary tests: `cargo test --workspace --all-targets`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0018-015-grouped-expression-typing.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0018-015-grouped-expression-typing.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0018-type-checking-core-accepted.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/type_check.rs`
- Implementation files:
  - `crates/newlang/src/type_check.rs`
- Documentation or checklist files:
  - `docs/tasks/M0018-015-grouped-expression-typing.md`
  - `docs/tasks/reviews/M0018-015-review.md`
  - `docs/tasks/soundness/M0018-015-soundness.md`

## Forbidden Changes

- Do not infer inner expression types.
- Do not resolve names or type literals in this task.
- Do not type calls, members, binary expressions, unary expressions, or if expressions.
- Do not change parser metadata.
- Do not add assignment, ownership, borrow, HIR, MIR, or backend behavior.
- Do not weaken or delete existing parser or type-checking tests.

## Ambiguities And Dependencies

- Integrating grouped expression typing into larger declaration initializer checks remains a later M0018 task.

## Execution Log

- 2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0018 grouped expression typing task.
- 2026-07-10 main_task=main-task test work phase=test-first result=pass notes=`cargo test --workspace --all-targets` failed before implementation with unresolved import `newlang::type_check::type_grouped_expressions`.
- 2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Added pure grouped expression type propagation from existing expression type records.
- 2026-07-10 main_task=main-task implementation phase=ordinary-tests result=pass notes=`cargo test --workspace --all-targets` passed with 151 tests.
- 2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0018-015-grouped-expression-typing.md` created a passing soundness report after ordinary tests.
- 2026-07-10 main_task=main-task review phase=review result=pass notes=Review approved grouped expression typing scope, spec compliance, and maintainability.
- 2026-07-10 main_task=main-task implementation phase=ci result=pass notes=`cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace --all-targets`; `sh docs/tests/m0018-type-checking-core-accepted.sh`; `sh docs/tests/m0002-workspace-ci.sh`.
