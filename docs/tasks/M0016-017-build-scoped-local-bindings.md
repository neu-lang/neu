# Task: M0016-017 Build scoped local bindings

## Task Metadata

- Task ID: `M0016-017`
- Milestone: `M0016`
- Milestone File: `docs/milestones/M0016-name-resolution-pass.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0016-017-build-scoped-local-bindings`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Build a local binding index by assigning parsed local bindings to their nearest containing block scope.

## Motivation

M0016 now has parser local binding metadata, local binding storage, and an AST-derived local scope tree. ADR-0026 says local `val` and `var` statements bind in the nearest containing lexical block. The next safe step is to use the scope tree to key local bindings by their containing block scope before implementing lookup.

## Scope

- Add a scoped local binding builder.
- Assign each parsed local binding to the nearest containing `Block` scope.
- Preserve insertion results and duplicate-name diagnostics.
- Allow the same name in nested child scopes.
- Preserve `val` and `var` binding kinds.

## Out Of Scope

- Local lookup.
- Declaration-order visibility.
- Expression name-reference resolution.
- Function parameter bindings.
- Pattern bindings.
- Import, cross-module, member, overload, extension, or type-directed lookup.

## Required Inputs

- Accepted ADR: `docs/adr/ADR-0026-name-resolution-policy.md`
- Parser local binding metadata from `crates/newlang/src/parser.rs`
- Local scope tree from `crates/newlang/src/name_resolution.rs`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Outer block binding uses the outer block scope.
  - Inner block binding uses the nested block scope.
  - Same local name in nested block scope is accepted as shadowing storage.
- Negative tests:
  - Duplicate local names in the same block scope produce `duplicate_name`.
- Adversarial tests:
  - Builder does not implement lookup.
  - Builder does not treat declaration-body scopes as local binding owners for block-local `val` or `var`.

## Test-First Gate

- Test files to update before implementation:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `build_scoped_local_binding_index` does not exist.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a builder that uses binding statement spans and scope owner spans to select the innermost containing block scope, then inserts the binding into `LocalBindingIndex` with that scope id.

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

## Execution Commands

- Generate tests: `update crates/newlang/tests/name_resolution.rs && update docs/tests/m0016-name-resolution-data-model.sh`
- Verify tests fail: `cargo test -p newlang --test name_resolution`
- Ordinary tests: `cargo test -p newlang --test name_resolution && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0016-017-build-scoped-local-bindings.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0016-017-build-scoped-local-bindings.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh && docs/tests/m0016-name-resolution-blocked.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Implementation files:
  - `crates/newlang/src/name_resolution.rs`
- Documentation or checklist files:
  - `docs/tasks/M0016-017-build-scoped-local-bindings.md`

## Forbidden Changes

- Do not implement local lookup.
- Do not resolve expression references.
- Do not add function parameter or pattern bindings.
- Do not modify accepted ADR-0026.

## Ambiguities And Dependencies

- Declaration-order visibility remains a later M0016 task.
- Actual expression name-reference resolution remains a later M0016 task.

## Execution Log

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0016 scoped local binding builder task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Updated name-resolution tests and M0016 data-model validator before adding build_scoped_local_binding_index.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=cargo test -p newlang --test name_resolution failed before implementation because build_scoped_local_binding_index was missing.
2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Added scoped local binding builder that assigns parsed local bindings to the nearest containing block scope.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=cargo test -p newlang --test name_resolution, M0016 data-model validator, and M0016 accepted-state validator passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created a soundness report after ordinary tests were recorded; concrete adversarial review found no lookup or declaration-order visibility.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0016-017-review.md approved scoped local binding construction pending final CI gate.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt, cargo clippy, cargo test, M0016 data-model/accepted/authority validators, M0015 validator, and M0002 validator passed.
```

## Handoff

- Next main task: `main-task implementation`
- Reason: `Add scoped local binding builder after tests fail.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `crates/newlang/src/name_resolution.rs`
