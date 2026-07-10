# Task: M0018-017 Assignment Statement Type Checking

## Task Metadata

- Task ID: `M0018-017`
- Milestone: `M0018`
- Milestone File: `docs/milestones/M0018-type-checking-core.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0018-017-assignment-statement-type-checking`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`

## Goal

Type check parsed assignment statements when both target and value expression types are known.

## Motivation

ADR-0027 includes assignment statements where both sides have known types and defines M0018 assignment compatibility as exact type identity with nullable exceptions. M0018-016 exposed parser metadata connecting assignment statements to target and value expression nodes.

## Scope

- Consume parser assignment statement metadata from M0018-016.
- Consume existing expression type side-table entries.
- Consume type arena records needed for nullable compatibility.
- Record successful `AssignmentCheck` entries for compatible assignments.
- Record `type_mismatch` diagnostics on the assigned value expression for incompatible assignments.
- Skip assignments when either side has no known expression type.
- Preserve assignment metadata order.

## Out Of Scope

- Typing target or value expressions.
- Resolving names.
- Inferring missing expression types.
- Assignment target legality rules.
- Variable mutability rules.
- Ownership moves.
- Borrow checking.
- HIR, MIR, or backend behavior.

## Required Inputs

- `ParsedAssignmentStatement` records.
- Existing `ExpressionType` records.
- Existing `TypeArena` records.
- ADR-0027 assignment compatibility rules.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Exact matching target and value types record assignment checks.
  - `Null` value is compatible with nullable target.
  - Non-null base value is compatible with its nullable wrapper target.
- Negative tests:
  - Mismatched known types produce `type_mismatch` on the value expression.
  - `Null` value is not compatible with non-null target.
  - Assignments with missing target or value types are skipped.
- Adversarial tests:
  - Assignment checking does not infer types, resolve names, enforce mutability, perform ownership checks, or type unsupported expressions.

## Test-First Gate

- Test files to edit before implementation:
  - `crates/compiler/tests/type_check.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Assignment statement type-checking entry point does not exist yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a pure helper that scans assignment metadata in order, finds target and value expression types in a supplied expression type slice, checks ADR-0027 compatibility against a supplied type arena, and records either an assignment check or a mismatch diagnostic.

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
- [x] No compiler behavior beyond assignment statement type checking is introduced.

## Execution Commands

- Generate tests: edit `crates/compiler/tests/type_check.rs`
- Verify tests fail: `cargo test --workspace --all-targets`
- Ordinary tests: `cargo test --workspace --all-targets`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0018-017-assignment-statement-type-checking.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0018-017-assignment-statement-type-checking.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0018-type-checking-core-accepted.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/compiler/tests/type_check.rs`
- Implementation files:
  - `crates/compiler/src/type_check.rs`
- Documentation or checklist files:
  - `docs/tasks/M0018-017-assignment-statement-type-checking.md`
  - `docs/tasks/reviews/M0018-017-review.md`
  - `docs/tasks/soundness/M0018-017-soundness.md`

## Forbidden Changes

- Do not infer target or value expression types.
- Do not resolve names.
- Do not enforce assignment target legality or mutability rules.
- Do not add ownership, borrow, HIR, MIR, or backend behavior.
- Do not type calls, members, binary expressions, unary expressions, or if expressions.
- Do not weaken or delete existing parser or type-checking tests.

## Ambiguities And Dependencies

- Integrating assignment statement checking into a larger whole-function type-check pass remains a later M0018 task.

## Execution Log

- 2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0018 assignment statement type-checking task.
- 2026-07-10 main_task=main-task test work phase=test-first result=pass notes=`cargo test --workspace --all-targets` failed before implementation with unresolved import `compiler::type_check::type_assignment_statements`.
- 2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Added assignment statement type checking for exact identity and ADR-0027 nullable exceptions.
- 2026-07-10 main_task=main-task implementation phase=ordinary-tests result=pass notes=`cargo test --workspace --all-targets` passed with 158 tests.
- 2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0018-017-assignment-statement-type-checking.md` created a passing soundness report after ordinary tests.
- 2026-07-10 main_task=main-task review phase=review result=pass notes=Review approved assignment type-checking scope, spec compliance, and maintainability.
- 2026-07-10 main_task=main-task implementation phase=ci result=pass notes=`cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace --all-targets`; `sh docs/tests/m0018-type-checking-core-accepted.sh`; `sh docs/tests/m0002-workspace-ci.sh`.
