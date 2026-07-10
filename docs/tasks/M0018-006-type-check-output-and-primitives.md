# Task: M0018-006 Add Type Check Output Tables And Primitive Identities

## Task Metadata

- Task ID: `M0018-006`
- Milestone: `M0018`
- Milestone File: `docs/milestones/M0018-type-checking-core.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0018-006-type-check-output-and-primitives`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
  - `docs/adr/ADR-0026-name-resolution-policy.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`

## Goal

Add the M0018 type-check report side-table shape and primitive type-checking identities required before expression and assignment checking can produce accepted typed output.

## Motivation

ADR-0027 requires expression type, declaration signature, and assignment check side tables plus primitive type-checking identities. Later M0018 tasks need these stable structures before they can implement literal, name, and assignment checking without inventing output contracts.

## Scope

- Add primitive type-checking identities for `Bool`, `Int`, `String`, `Unit`, and `Null`.
- Store primitive identities as type records with no ABI or layout meaning.
- Add expression type table records keyed by `AstNodeId`.
- Add declaration signature table records keyed by `AstNodeId`.
- Add assignment check table records keyed by `AstNodeId`.
- Preserve insertion order and lookup by node id for all new side tables.

## Out Of Scope

- Parsing annotations into type records.
- Literal expression typing.
- Name expression typing.
- Assignment compatibility.
- Type mismatch diagnostics.
- Direct calls or function type application.
- HIR, MIR, backend, ownership, or borrow checking.

## Required Inputs

- Accepted ADR-0027 type output shape.
- Existing `types` module from M0017.
- Existing `type_check` blocker report scaffold from earlier M0018 tasks.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Primitive type records preserve all five ADR-0027 primitive identities.
  - Type-check report records expression types, declaration signatures, and assignment checks in insertion order.
  - Type-check report lookups return records by `AstNodeId`.
- Negative tests:
  - Primitive type records expose no ABI, layout, width, signedness, or backend metadata.
  - Empty reports do not synthesize successful type entries.
- Diagnostic tests:
  - Existing ambiguous blocker diagnostics continue to work.
- Adversarial tests:
  - The task does not implement literal typing, assignment compatibility, direct calls, ownership, borrow checking, HIR, MIR, or backend behavior.

## Test-First Gate

- Test files to edit before implementation:
  - `crates/compiler/tests/types.rs`
  - `crates/compiler/tests/type_check.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `PrimitiveType`, primitive `TypeRecord`s, expression type entries, declaration signature entries, and assignment check entries do not exist yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Extend the existing M0017 type model with primitive type-checking identities, then extend the M0018 type-check report with side-table records only. Keep behavior as data-model support; do not add a traversal-based checker in this task.

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
- [x] No compiler behavior beyond M0018 output side tables and primitive identities is introduced.

## Execution Commands

- Generate tests: edit `crates/compiler/tests/types.rs` and `crates/compiler/tests/type_check.rs`
- Verify tests fail: `cargo test --workspace --all-targets`
- Ordinary tests: `cargo test --workspace --all-targets`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0018-006-type-check-output-and-primitives.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0018-006-type-check-output-and-primitives.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0018-type-checking-core-accepted.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/compiler/tests/types.rs`
  - `crates/compiler/tests/type_check.rs`
- Implementation files:
  - `crates/compiler/src/types.rs`
  - `crates/compiler/src/type_check.rs`
- Documentation or checklist files:
  - `docs/tasks/M0018-006-type-check-output-and-primitives.md`
  - `docs/tasks/reviews/M0018-006-review.md`
  - `docs/tasks/soundness/M0018-006-soundness.md`

## Forbidden Changes

- Do not implement expression traversal.
- Do not infer literal types.
- Do not check assignments.
- Do not add direct call or function type application behavior.
- Do not add ABI, layout, integer width, signedness, or backend meaning to primitive type identities.
- Do not weaken or delete existing M0018 tests.

## Ambiguities And Dependencies

- Literal typing depends on this task and must be handled in a later M0018 task.
- Assignment compatibility depends on this task and must be handled in a later M0018 task.
- Direct calls and structural function type application remain deferred by ADR-0027.

## Execution Log

- 2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0018 primitive identity and type-check output table task.
- 2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Added primitive type identity and type-check side-table tests before implementation.
- 2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=`cargo test --workspace --all-targets` failed because `PrimitiveType`, primitive `TypeRecord`s, `ExpressionType`, `DeclarationSignature`, `AssignmentCheck`, and empty `TypeCheckReport::new` are not implemented yet.
- 2026-07-10 main_task=main-task implementation phase=ordinary-tests result=pass notes=Added primitive type records and type-check side-table records only; `cargo fmt --all --check`, `cargo test --workspace --all-targets`, `cargo clippy --workspace --all-targets -- -D warnings`, `sh docs/tests/m0018-type-checking-core-accepted.sh`, and `sh docs/tests/m0002-workspace-ci.sh` passed.
- 2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0018-006-type-check-output-and-primitives.md` created a passing soundness report; concrete adversarial review found no scope expansion.
- 2026-07-10 main_task=main-task review phase=review result=pass notes=`docs/scripts/review-task.sh docs/tasks/M0018-006-type-check-output-and-primitives.md` created review artifact; concrete review approved against SPEC, ADR-0027, and M0018.
- 2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=Final CI gate passed after review and soundness reports.
