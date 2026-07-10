# Task: M0018-018 Accepted Expression Type Composition

## Task Metadata

- Task ID: `M0018-018`
- Milestone: `M0018`
- Milestone File: `docs/milestones/M0018-type-checking-core.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0018-018-accepted-expression-type-composition`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`

## Goal

Compose accepted M0018 expression typing inputs into one expression type side table.

## Motivation

M0018 now has separate helpers for literal expressions, resolved name expressions, and grouped expressions. Later declaration initializer and assignment checking need a single accepted expression type table without reimplementing those rules.

## Scope

- Consume parser literal expression metadata.
- Consume parser grouped expression metadata.
- Consume M0016 resolved name records.
- Consume explicit known symbol type records.
- Emit one expression type side table for accepted expressions.
- Type grouped expressions from the same composed expression table.
- Skip expressions whose required input type is missing.

## Out Of Scope

- Declaration checking.
- Assignment checking.
- Type inference.
- Name resolution.
- Unsupported expression diagnostics.
- Call, member, binary, unary, or value-producing `if` expression typing.
- Ownership, borrow, HIR, MIR, or backend behavior.

## Required Inputs

- `ParsedLiteralExpression` records.
- `ParsedGroupedExpression` records.
- `ResolutionTable` records.
- `KnownSymbolType` records.
- ADR-0027 included expression rules.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Literal, resolved name, and grouped expression types appear in one report.
  - Nested grouped expressions are typed when their inner expression becomes known during composition.
  - The helper returns the bootstrap primitive type arena used for literal typing.
- Negative tests:
  - Unknown resolved names are skipped.
  - Grouped expressions with unknown inner expressions are skipped.
  - No declaration signatures, assignment checks, or diagnostics are synthesized.
- Adversarial tests:
  - Composition does not infer types, run name resolution, type unsupported expressions, or check assignments.

## Test-First Gate

- Test files to edit before implementation:
  - `crates/compiler/tests/type_check.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Accepted expression composition entry point does not exist yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a helper that builds the primitive arena, records literal expression types, records resolved name expression types from supplied known symbol types, and then propagates grouped expression types from the composed report until no new grouped expression type can be added.

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
- [x] No compiler behavior beyond accepted expression type composition is introduced.

## Execution Commands

- Generate tests: edit `crates/compiler/tests/type_check.rs`
- Verify tests fail: `cargo test --workspace --all-targets`
- Ordinary tests: `cargo test --workspace --all-targets`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0018-018-accepted-expression-type-composition.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0018-018-accepted-expression-type-composition.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0018-type-checking-core-accepted.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/compiler/tests/type_check.rs`
- Implementation files:
  - `crates/compiler/src/type_check.rs`
- Documentation or checklist files:
  - `docs/tasks/M0018-018-accepted-expression-type-composition.md`
  - `docs/tasks/reviews/M0018-018-review.md`
  - `docs/tasks/soundness/M0018-018-soundness.md`

## Forbidden Changes

- Do not infer expression types.
- Do not run name resolution.
- Do not type calls, members, binary expressions, unary expressions, or value-producing `if` expressions.
- Do not add assignment checks.
- Do not add declaration signatures.
- Do not add ownership, borrow, HIR, MIR, or backend behavior.
- Do not weaken or delete existing parser or type-checking tests.

## Ambiguities And Dependencies

- Integrating composed expression typing into declaration and assignment orchestration remains a later M0018 task.

## Execution Log

- 2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0018 accepted expression type composition task.
- 2026-07-10 main_task=main-task test work phase=test-first result=pass notes=`cargo test --workspace --all-targets` failed before implementation with unresolved import `compiler::type_check::type_m0018_accepted_expressions`.
- 2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Added accepted expression composition for literals, resolved names, and grouped expressions.
- 2026-07-10 main_task=main-task implementation phase=ordinary-tests result=pass notes=`cargo test --workspace --all-targets` passed with 161 tests.
- 2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0018-018-accepted-expression-type-composition.md` created a passing soundness report after ordinary tests.
- 2026-07-10 main_task=main-task review phase=review result=pass notes=Review approved accepted expression composition scope, spec compliance, and maintainability.
- 2026-07-10 main_task=main-task implementation phase=ci result=pass notes=`cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace --all-targets`; `sh docs/tests/m0018-type-checking-core-accepted.sh`; `sh docs/tests/m0002-workspace-ci.sh`.
