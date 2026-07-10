# Task: M0018-019 Accepted Local Initializer Checks

## Task Metadata

- Task ID: `M0018-019`
- Milestone: `M0018`
- Milestone File: `docs/milestones/M0018-type-checking-core.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0018-019-accepted-local-initializer-checks`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`

## Goal

Check local declaration initializers using all accepted M0018 expression types.

## Motivation

M0018-011 checked primitive local declaration initializers only when the initializer was a literal. M0018-018 now composes accepted expression typing for literals, resolved names, and grouped expressions, so local declaration initializer checks can use that composed table without inventing inference.

## Scope

- Consume local declaration metadata.
- Consume primitive explicit annotation metadata.
- Consume literal, grouped expression, and resolved name expression inputs.
- Consume explicit known symbol type records.
- Record declaration signatures for known primitive annotations.
- Record expression types for accepted initializer expressions.
- Record assignment checks or `type_mismatch` diagnostics for typed local declaration initializers.
- Skip declarations with unknown annotations or untyped initializers.

## Out Of Scope

- Inferring declaration types.
- Running name resolution.
- Deriving known symbol types.
- Assignment statement checking.
- Unsupported expression diagnostics.
- Call, member, binary, unary, or value-producing `if` expression typing.
- Ownership, borrow, HIR, MIR, or backend behavior.

## Required Inputs

- `ParsedLocalDeclaration` records.
- `ParsedTypeNameReference` records.
- `ParsedLiteralExpression` records.
- `ParsedGroupedExpression` records.
- `ResolutionTable` records.
- `KnownSymbolType` records.
- ADR-0027 assignment compatibility rules.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Literal initializers still record assignment checks.
  - Resolved name initializers record assignment checks when compatible.
  - Grouped resolved-name initializers record assignment checks when compatible.
- Negative tests:
  - Mismatched accepted initializers produce `type_mismatch` on the initializer expression.
  - Unknown annotations are skipped.
  - Untyped initializers are skipped.
- Adversarial tests:
  - The helper does not infer declarations, run name resolution, derive symbol types, type unsupported expressions, or check assignment statements.

## Test-First Gate

- Test files to edit before implementation:
  - `crates/compiler/tests/type_check.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Accepted local initializer checking entry point does not exist yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a helper that builds primitive annotation signatures, composes accepted expression types, and applies existing M0018 assignment compatibility to declaration initializers with known target and initializer types.

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
- [x] No compiler behavior beyond accepted local initializer checking is introduced.

## Execution Commands

- Generate tests: edit `crates/compiler/tests/type_check.rs`
- Verify tests fail: `cargo test --workspace --all-targets`
- Ordinary tests: `cargo test --workspace --all-targets`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0018-019-accepted-local-initializer-checks.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0018-019-accepted-local-initializer-checks.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0018-type-checking-core-accepted.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/compiler/tests/type_check.rs`
- Implementation files:
  - `crates/compiler/src/type_check.rs`
- Documentation or checklist files:
  - `docs/tasks/M0018-019-accepted-local-initializer-checks.md`
  - `docs/tasks/reviews/M0018-019-review.md`
  - `docs/tasks/soundness/M0018-019-soundness.md`

## Forbidden Changes

- Do not infer declaration types.
- Do not run name resolution or derive known symbol types.
- Do not type calls, members, binary expressions, unary expressions, or value-producing `if` expressions.
- Do not check assignment statements.
- Do not add ownership, borrow, HIR, MIR, or backend behavior.
- Do not weaken or delete existing parser or type-checking tests.

## Ambiguities And Dependencies

- Integrating this helper with automatic known-symbol derivation remains a later M0018 task.

## Execution Log

- 2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0018 accepted local initializer checking task.
- 2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Added accepted local initializer tests for literal, resolved name, grouped resolved-name, mismatch, unknown annotation, and untyped initializer behavior before implementation.
- 2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=`cargo test --workspace --all-targets` failed before implementation with unresolved import `compiler::type_check::type_m0018_local_declaration_initializers`.
- 2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Added `type_m0018_local_declaration_initializers` to compose accepted expression types, record primitive declaration signatures, and check typed local initializers.
- 2026-07-10 main_task=main-task implementation phase=ordinary-tests result=pass notes=`cargo test --workspace --all-targets` passed with 163 tests.
- 2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0018-019-accepted-local-initializer-checks.md` passed after ordinary tests.
- 2026-07-10 main_task=main-task review phase=review result=pass notes=Review approved against `docs/SPEC.md`, ADR-0027, ADR-0026, ADR-0024, and `docs/milestones/M0018-type-checking-core.md`.
- 2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, `sh docs/tests/m0018-type-checking-core-accepted.sh`, and `sh docs/tests/m0002-workspace-ci.sh` passed.
