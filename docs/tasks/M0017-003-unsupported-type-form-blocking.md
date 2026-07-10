# Task: M0017-003 Block Unsupported Type Forms In Type Representation

## Task Metadata

- Task ID: `M0017-003`
- Milestone: `M0017`
- Milestone File: `docs/milestones/M0017-type-representation.md`
- Status: `complete`
- Owner Agent: `Implementer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0017-003-unsupported-type-form-blocking`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
- Milestone: `docs/milestones/M0017-type-representation.md`

## Goal

Add a type-representation diagnostic model that can explicitly block unsupported type forms instead of silently representing or guessing them.

## Motivation

M0017 requires unsupported type forms to be rejected or marked blocked. ADR-0023 lists accepted bootstrap type syntax and concrete deferrals; the type representation layer needs a stable way to record those unsupported forms when later lowering encounters them.

## Scope

- Add an `UnsupportedTypeForm` enum for source-of-truth deferred type forms.
- Add a `TypeDiagnosticKind::UnsupportedTypeForm` diagnostic category.
- Add a `TypeDiagnostic` carrying the unsupported form and AST node.
- Add a helper for constructing unsupported-form diagnostics.
- Add tests proving deferred forms are represented as blocked diagnostics, not type records.
- Add a validator for this slice.
- Mark the M0017 unsupported type-form checklist item complete if the slice satisfies it.

## Out Of Scope

- Parser behavior changes.
- Type lowering from parsed AST.
- Type inference.
- Constraint solving.
- Primitive scalar catalog.
- Nullable flow checks.
- Ownership capabilities.
- Layout, ABI, HIR, MIR, or backend behavior.

## Required Inputs

- Milestone: `docs/milestones/M0017-type-representation.md`
- ADR-0015 diagnostic obligations.
- ADR-0023 accepted and deferred type syntax.
- Existing M0017 type model.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Unsupported type forms preserve the deferred form and AST node.
  - Concrete ADR-0023 deferred forms are covered.
  - Unsupported diagnostics are separate from `TypeRecord` storage.
- Negative tests:
  - Focused Rust tests fail before implementation because `UnsupportedTypeForm`, `TypeDiagnostic`, and `TypeDiagnosticKind` do not exist.
- Diagnostic tests:
  - Unit tests check diagnostic kind, form, and primary AST node.
- Adversarial tests:
  - Confirm this task does not add parser changes, type lowering, inference, constraint solving, primitive catalog, ownership capability, layout, ABI, HIR, MIR, or backend logic.

## Test-First Gate

- Test files to create before implementation:
  - `crates/newlang/tests/types.rs`
  - `docs/tests/m0017-unsupported-type-form-blocking.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `UnsupportedTypeForm`, `TypeDiagnostic`, and `TypeDiagnosticKind` do not exist yet.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a small diagnostic-only model in `types.rs`. Do not create `TypeKind::Unsupported`, because blocked forms should not become usable types.

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
- [x] Milestone unsupported-form checklist is updated.

## Execution Commands

- Generate tests: `edit crates/newlang/tests/types.rs and create docs/tests/m0017-unsupported-type-form-blocking.sh`
- Verify tests fail: `cargo test -p newlang --test types`
- Ordinary tests: `cargo test -p newlang --test types && sh docs/tests/m0017-unsupported-type-form-blocking.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0017-003-unsupported-type-form-blocking.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0017-003-unsupported-type-form-blocking.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0017-unsupported-type-form-blocking.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/types.rs`
  - `docs/tests/m0017-unsupported-type-form-blocking.sh`
- Implementation files:
  - `crates/newlang/src/types.rs`
- Documentation or checklist files:
  - `docs/milestones/M0017-type-representation.md`
  - `docs/tasks/M0017-003-unsupported-type-form-blocking.md`
  - `docs/tasks/reviews/M0017-003-review.md`
  - `docs/tasks/soundness/M0017-003-soundness.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not modify parser behavior.
- Do not add `TypeKind::Unsupported`.
- Do not implement type lowering or type checking.
- Do not weaken or delete failing tests without reviewer approval.
- Do not introduce language semantics not present in `docs/SPEC.md` or `docs/adr/`.

## Ambiguities And Dependencies

- Primitive scalar categories remain unspecified and require a separate ambiguity or source-of-truth task.
- Lowering parsed unsupported syntax into these diagnostics is deferred until a type lowering milestone or task.

## Execution Log

- 2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Task references only M0017 and blocks deferred type forms as diagnostics.
- 2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=`cargo test -p newlang --test types` failed because `UnsupportedTypeForm`, `TypeDiagnostic`, and `TypeDiagnosticKind` did not exist.
- 2026-07-10 agent=Implementer phase=ordinary-tests result=pass notes=`cargo test -p newlang --test types` passed after adding diagnostic-only unsupported type-form blocking; validator initially failed only because the milestone checklist had not yet been updated.
- 2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0017-003-unsupported-type-form-blocking.md` created a passing soundness report.
- 2026-07-10 agent=Reviewer phase=review result=pass notes=`docs/scripts/review-task.sh docs/tasks/M0017-003-unsupported-type-form-blocking.md` created review and concrete review approved after source-of-truth comparison.
- 2026-07-10 agent=Build-Engineer phase=ci result=pass notes=`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, `sh docs/tests/m0017-unsupported-type-form-blocking.sh`, `sh docs/tests/m0017-nullable-type-representation.sh`, `sh docs/tests/m0017-type-identity-model.sh`, `sh docs/tests/m0016-name-resolution-data-model.sh`, and `sh docs/tests/m0002-workspace-ci.sh` passed.
