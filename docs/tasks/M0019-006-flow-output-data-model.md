# Task: M0019-006 Add Flow Output Data Model

## Task Metadata

- Task ID: `M0019-006`
- Milestone: `M0019`
- Milestone File: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0019-006-flow-output-data-model`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
  - `docs/adr/ADR-0027-type-checking-core.md`
- Milestone: `docs/milestones/M0019-nullability-and-flow-typing.md`

## Goal

Add the M0019 flow output and diagnostic data model to the type-checking report without implementing branch recognition or nullability checking.

## Motivation

ADR-0028 says M0019 extends M0018 side-table output with refinement records, refined expression type entries, and flow diagnostics. Later implementation tasks need a stable data model before they can recognize null tests or apply refinements.

## Scope

- Add flow diagnostic kinds and stable rule identifiers required by ADR-0028.
- Add refinement records that preserve original local binding, original nullable type, refined non-null type, branch region, and originating null-test expression.
- Add refined expression type entries for per-use refined type views.
- Extend `TypeCheckReport` to record and expose flow refinements and refined expression types.
- Add focused Rust tests and a docs validator.

## Out Of Scope

- Recognizing null-test expressions.
- Traversing `if` branches.
- Applying smart casts.
- Diagnosing nullable misuse in programs.
- Implementing mutation invalidation.
- Updating examples beyond the already accepted source-language example.
- HIR, MIR, ownership, borrow, coroutine, unsafe, FFI, or backend behavior.

## Required Inputs

- Accepted ADR: `docs/adr/ADR-0028-nullability-and-flow-typing.md`
- M0018 type-check report model.
- Type model with nullable wrappers from M0017.
- Local binding identity model from M0016.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Flow diagnostic constructors preserve kind, stable rule identifier, node, expected type, and actual type.
  - Flow diagnostic stable rule identifiers cover ADR-0028 examples.
  - Type check report records flow refinements in insertion order.
  - Type check report records refined expression type entries in insertion order and lookup preserves the first inserted view.
- Negative tests:
  - This task does not recognize null tests or implement smart casts.
  - This task does not add HIR, MIR, or backend modules.
- Diagnostic tests:
  - `invalid_nullable_use`, `invalidated_refinement`, `unsupported_flow_rule`, and `ambiguous_flow_rule` are represented as diagnostic kinds.
- Adversarial tests:
  - The data model preserves original nullable and refined non-null types separately and does not rewrite declaration signatures.

## Test-First Gate

- Test files to create before implementation:
  - `crates/newlang/tests/type_check.rs`
  - `docs/tests/m0019-flow-output-data-model.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Flow diagnostic and refinement data model types do not exist.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Extend the existing type-checking report data structures only. Keep behavior passive: constructors, recorders, accessors, and no semantic traversal.

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

- Generate tests: `edit crates/newlang/tests/type_check.rs and create docs/tests/m0019-flow-output-data-model.sh`
- Verify tests fail: `cargo test -p newlang --test type_check m0019_flow`
- Ordinary tests: `cargo test -p newlang --test type_check m0019_flow && sh docs/tests/m0019-flow-output-data-model.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0019-006-flow-output-data-model.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0019-006-flow-output-data-model.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0019-flow-output-data-model.sh && sh docs/tests/m0019-nullability-flow-accepted.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/type_check.rs`
  - `docs/tests/m0019-flow-output-data-model.sh`
- Implementation files:
  - `crates/newlang/src/type_check.rs`
- Documentation or checklist files:
  - `docs/tasks/M0019-006-flow-output-data-model.md`
  - `docs/tasks/reviews/M0019-006-review.md`
  - `docs/tasks/soundness/M0019-006-soundness.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not implement null-test recognition.
- Do not implement branch traversal.
- Do not implement smart casts.
- Do not implement nullable misuse checking.
- Do not add HIR, MIR, or backend behavior.

## Ambiguities And Dependencies

- Branch recognition depends on future parser or analysis metadata that can identify condition operands and branch regions.
- This task intentionally provides data-model targets for later behavior.

## Execution Log

- 2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0019 flow output data model task.
- 2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Added focused Rust tests and docs validator before implementing flow output data model.
- 2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=`cargo test -p newlang --test type_check m0019_flow` failed because flow diagnostic and refinement output data model APIs were missing.
- 2026-07-10 main_task=main-task implementation phase=ordinary-tests result=pass notes=Added passive M0019 flow diagnostic, refinement, and refined expression output data model; `cargo test -p newlang --test type_check`, `cargo test -p newlang --test type_check m0019_flow`, and `sh docs/tests/m0019-flow-output-data-model.sh` passed.
- 2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0019-006-flow-output-data-model.md` created a passing soundness report.
- 2026-07-10 main_task=main-task review phase=review result=pass notes=`docs/scripts/review-task.sh docs/tasks/M0019-006-flow-output-data-model.md` created review report; concrete review approved data-model-only scope.
- 2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, `sh docs/tests/m0019-flow-output-data-model.sh`, M0019 accepted validator chain, and `sh docs/tests/m0002-workspace-ci.sh` passed. Examples skipped because this task adds no source-language behavior.

## Handoff

- Next main task: `main-task test work`
- Reason: `Verify tests fail before implementation.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
  - `crates/newlang/src/type_check.rs`
