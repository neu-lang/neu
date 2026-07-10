# Task: M0017-001 Create Type Identity Representation

## Task Metadata

- Task ID: `M0017-001`
- Milestone: `M0017`
- Milestone File: `docs/milestones/M0017-type-representation.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0017-001-type-identity-model`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0016-generics-and-parametric-polymorphism.md`
- Milestone: `docs/milestones/M0017-type-representation.md`

## Goal

Create the first internal type representation slice for nominal type identity and generic placeholder identity without implementing type checking.

## Motivation

M0017 needs a shared type model before M0018 can check expressions or declarations. This task establishes stable type IDs and identity records while avoiding unresolved primitive and checking semantics.

## Scope

- Add a type module exported by the compiler crate.
- Add stable insertion-ordered type IDs.
- Represent nominal type identity using module, package, and symbol identity.
- Represent generic placeholder identity using its declaring AST node and symbol.
- Add unit tests proving identity, ordering, and distinct namespaces.
- Add a milestone validator for this task slice.

## Out Of Scope

- Primitive scalar catalog or numeric semantics.
- Nullable type wrappers.
- Type inference.
- Constraint solving.
- Interface or protocol conformance.
- Ownership capabilities.
- Layout, ABI, HIR, MIR, or backend behavior.

## Required Inputs

- Milestone: `docs/milestones/M0017-type-representation.md`
- Name and package identity from M0014-M0016.
- Symbol identity from M0015.
- ADR-0006 nullable type decision.
- ADR-0010 nominal type system decision.
- ADR-0016 generic placeholder and constraint direction.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Type IDs are stable and insertion ordered.
  - Nominal identity includes module, package, declaration node, and symbol.
  - Distinct packages produce distinct nominal identities.
  - Generic placeholder identity preserves declaration node and symbol.
- Negative tests:
  - The validator fails before implementation because `crates/newlang/src/types.rs` and `crates/newlang/tests/types.rs` are absent.
- Diagnostic tests:
  - Not applicable; this slice adds representation only.
- Adversarial tests:
  - Confirm this task does not introduce inference, constraint solving, ownership capability, layout, ABI, HIR, MIR, backend, or primitive catalog logic.

## Test-First Gate

- Test files to create before implementation:
  - `crates/newlang/tests/types.rs`
  - `docs/tests/m0017-type-identity-model.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - The public `newlang::types` module does not exist yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add the smallest type representation module that can store and retrieve type records by stable IDs. Limit variants to nominal type identity and generic parameter identity because the primitive catalog and nullable lowering require separate M0017 tasks.

## Acceptance Criteria

- [ ] Task references exactly one milestone.
- [ ] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing change.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] main-task review compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [x] Milestone checklist is updated only if this task fully satisfies an item.

## Execution Commands

- Generate tests: `create crates/newlang/tests/types.rs docs/tests/m0017-type-identity-model.sh`
- Verify tests fail: `cargo test -p newlang --test types`
- Ordinary tests: `cargo test -p newlang --test types && docs/tests/m0017-type-identity-model.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0017-001-type-identity-model.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0017-001-type-identity-model.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0017-type-identity-model.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/types.rs`
  - `docs/tests/m0017-type-identity-model.sh`
- Implementation files:
  - `crates/newlang/src/types.rs`
  - `crates/newlang/src/lib.rs`
- Documentation or checklist files:
  - `docs/tasks/M0017-001-type-identity-model.md`
  - `docs/tasks/reviews/M0017-001-type-identity-model.md`
  - `docs/tasks/soundness/M0017-001-type-identity-model.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not implement type checking.
- Do not introduce a primitive scalar catalog in this task.
- Do not implement nullable type representation in this task.
- Do not weaken or delete failing tests without main-task review approval.
- Do not introduce language semantics not present in `docs/SPEC.md` or `docs/adr/`.

## Ambiguities And Dependencies

- The primitive scalar set may be unspecified by current source-of-truth documents and is intentionally deferred.
- Nullable representation is required by M0017 but is a separate task because this task only establishes identity storage.
- Generic constraints and capability bounds are deferred to later M0017/M0020 tasks.

## Execution Log

- 2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Task references only M0017 and avoids unresolved primitive semantics.
- 2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=`cargo test -p newlang --test types` failed because `newlang::types` did not exist.
- 2026-07-10 main_task=main-task implementation phase=ordinary-tests result=pass notes=`cargo test -p newlang --test types` and `sh docs/tests/m0017-type-identity-model.sh` passed.
- 2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0017-001-type-identity-model.md` created a passing soundness report.
- 2026-07-10 main_task=main-task review phase=review result=pass notes=`docs/scripts/review-task.sh docs/tasks/M0017-001-type-identity-model.md` created review and concrete review approved after source-of-truth comparison.
- 2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, `sh docs/tests/m0017-type-identity-model.sh`, `sh docs/tests/m0016-name-resolution-data-model.sh`, and `sh docs/tests/m0002-workspace-ci.sh` passed.
