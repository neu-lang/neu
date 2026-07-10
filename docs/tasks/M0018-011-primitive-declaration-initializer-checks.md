# Task: M0018-011 Check Primitive Local Declaration Initializers

## Task Metadata

- Task ID: `M0018-011`
- Milestone: `M0018`
- Milestone File: `docs/milestones/M0018-type-checking-core.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0018-011-primitive-declaration-initializer-checks`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`

## Goal

Check local declaration initializers when both the declaration annotation and initializer expression have known primitive M0018 types.

## Motivation

M0018 requires well-typed fixtures to pass and ill-typed fixtures to fail with source-spanned diagnostics. Prior tasks can type primitive literal expressions and primitive local declaration annotations. This task connects those side tables for the first declaration initializer checks.

## Scope

- Accept parser local declaration metadata, parser primitive annotation names, and parser literal metadata.
- Record declaration signatures for primitive annotations.
- Record expression types for literal initializers.
- Record assignment checks for initializer types that exactly match the declaration signature.
- Record `type_mismatch` diagnostics when a typed initializer does not match the declaration signature.
- Use the initializer expression node as the diagnostic primary node.

## Out Of Scope

- Nullable assignment compatibility.
- Assignment statements outside declarations.
- Name expression typing.
- Grouped expression typing.
- Non-literal initializer typing.
- Nominal type resolution.
- Generic solving.
- Direct calls or function type application.
- Ownership, borrow checking, HIR, MIR, or backend behavior.

## Required Inputs

- Parser local declaration metadata from M0018-009.
- Parser literal metadata from M0018-008.
- Primitive declaration signatures from M0018-010.
- Literal expression typing from M0018-007.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Matching primitive literal initializers record assignment checks.
  - Declaration signatures and expression type entries are preserved.
  - Assignment checks preserve local declaration order.
- Negative tests:
  - Mismatched primitive literal initializers record `type_mismatch`.
  - Mismatched declarations do not record assignment checks.
  - Untyped initializers and untyped declarations are skipped rather than guessed.
- Diagnostic tests:
  - `type_mismatch` diagnostics preserve primary initializer node, expected type, and actual type.
- Adversarial tests:
  - Initializer checks do not implement nullable compatibility, non-literal typing, nominal lookup, generic solving, ownership, borrow checking, HIR, MIR, or backend behavior.

## Test-First Gate

- Test files to edit before implementation:
  - `crates/newlang/tests/type_check.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Primitive local declaration initializer check entry point and `type_mismatch` diagnostics do not exist yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a small checker that builds the existing primitive declaration signature and literal expression side tables in one report, then compares known initializer type IDs exactly against known declaration signature type IDs.

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
- [x] No compiler behavior beyond primitive literal declaration initializer checks is introduced.

## Execution Commands

- Generate tests: edit `crates/newlang/tests/type_check.rs`
- Verify tests fail: `cargo test --workspace --all-targets`
- Ordinary tests: `cargo test --workspace --all-targets`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0018-011-primitive-declaration-initializer-checks.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0018-011-primitive-declaration-initializer-checks.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0018-type-checking-core-accepted.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/type_check.rs`
- Implementation files:
  - `crates/newlang/src/type_check.rs`
- Documentation or checklist files:
  - `docs/tasks/M0018-011-primitive-declaration-initializer-checks.md`
  - `docs/tasks/reviews/M0018-011-review.md`
  - `docs/tasks/soundness/M0018-011-soundness.md`

## Forbidden Changes

- Do not add nullable assignment compatibility.
- Do not type name, grouped, call, member, binary, or unary initializers.
- Do not resolve nominal or generic annotations.
- Do not add ownership, borrow, HIR, MIR, or backend behavior.
- Do not weaken or delete existing M0018 tests.

## Ambiguities And Dependencies

- Nullable compatibility remains a later M0018 task because nullable annotation resolution is not implemented yet.
- Assignment statements remain a later M0018 task.
- Name expression typing remains a later M0018 task.

## Execution Log

- 2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0018 primitive declaration initializer check task.
- 2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Added primitive declaration initializer success and mismatch tests before implementation.
- 2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=`cargo test --workspace --all-targets` failed because `type_primitive_local_initializer_declarations` and `TypeCheckDiagnosticKind::TypeMismatch` do not exist yet.
- 2026-07-10 main_task=main-task implementation phase=ordinary-tests result=pass notes=Added exact primitive literal initializer checks only; `cargo fmt --all --check`, `cargo test --workspace --all-targets`, `cargo clippy --workspace --all-targets -- -D warnings`, `sh docs/tests/m0018-type-checking-core-accepted.sh`, and `sh docs/tests/m0002-workspace-ci.sh` passed.
- 2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0018-011-primitive-declaration-initializer-checks.md` created a passing soundness report; concrete adversarial review found no scope expansion.
- 2026-07-10 main_task=main-task review phase=review result=pass notes=`docs/scripts/review-task.sh docs/tasks/M0018-011-primitive-declaration-initializer-checks.md` created review artifact; concrete review approved against SPEC, ADR-0027, and M0018.
- 2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=Final CI gate passed after review and soundness reports.
