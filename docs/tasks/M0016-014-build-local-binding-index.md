# Task: M0016-014 Build local binding index from parser metadata

## Task Metadata

- Task ID: `M0016-014`
- Milestone: `M0016`
- Milestone File: `docs/milestones/M0016-name-resolution-pass.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0016-014-build-local-binding-index`

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

Build a `LocalBindingIndex` from parsed local binding metadata for a caller-supplied lexical scope id.

## Motivation

M0016-012 added local binding storage and M0016-013 added parser metadata for local `val` and `var` binding names. The next safe step is to convert parser metadata into storage while still deferring lexical scope construction and lookup.

## Scope

- Add a builder for parsed local binding metadata.
- Intern local binding names through the existing symbol interner.
- Insert local bindings into `LocalBindingIndex` with a caller-supplied `LocalScopeId`.
- Preserve insert results for later diagnostics and review.
- Emit `duplicate_name` diagnostics for duplicate binding keys within the supplied scope.

## Out Of Scope

- Lexical scope discovery.
- Assigning different scope ids to nested blocks.
- Local lookup.
- Declaration-order visibility.
- Expression name-reference resolution.
- Function parameter bindings.
- Pattern bindings.
- Import, cross-module, member, overload, extension, or type-directed lookup.

## Required Inputs

- Accepted ADR: `docs/adr/ADR-0026-name-resolution-policy.md`
- Local binding storage from `crates/compiler/src/name_resolution.rs`
- Parser local binding metadata from `crates/compiler/src/parser.rs`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Builder interns local binding names and preserves parsed order.
  - Builder preserves `val` and `var` binding kinds.
  - Builder records inserted results.
- Negative tests:
  - Duplicate local binding metadata in the same supplied scope records a duplicate insert and `duplicate_name` diagnostic.
- Adversarial tests:
  - Builder does not discover lexical scopes.
  - Builder does not implement lookup or declaration-order visibility.

## Test-First Gate

- Test files to update before implementation:
  - `crates/compiler/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Local binding index builder APIs do not exist.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a `LocalBindingIndexBuild` result type and a `build_local_binding_index` function that accepts parsed local binding metadata, one explicit local scope id, and the symbol interner.

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

- Generate tests: `update crates/compiler/tests/name_resolution.rs && update docs/tests/m0016-name-resolution-data-model.sh`
- Verify tests fail: `cargo test -p compiler --test name_resolution`
- Ordinary tests: `cargo test -p compiler --test name_resolution && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0016-014-build-local-binding-index.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0016-014-build-local-binding-index.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh && docs/tests/m0016-name-resolution-blocked.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/compiler/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Implementation files:
  - `crates/compiler/src/name_resolution.rs`
- Documentation or checklist files:
  - `docs/tasks/M0016-014-build-local-binding-index.md`

## Forbidden Changes

- Do not assign lexical scope ids from parser structure.
- Do not implement local lookup.
- Do not traverse expression AST.
- Do not add function parameter or pattern bindings.
- Do not modify accepted ADR-0026.

## Ambiguities And Dependencies

- Real lexical scope assignment remains a later M0016 task.
- Declaration-order visibility remains a later M0016 task.

## Execution Log

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0016 local binding index builder task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Updated name-resolution tests and M0016 data-model validator before adding local binding index builder APIs.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=cargo test -p compiler --test name_resolution failed before implementation because build_local_binding_index was missing.
2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Added LocalBindingIndexBuild and build_local_binding_index for parsed local binding metadata in a caller-supplied scope.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=cargo test -p compiler --test name_resolution, M0016 data-model validator, and M0016 accepted-state validator passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created a soundness report after ordinary tests were recorded; concrete adversarial review found no scope discovery or lookup.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0016-014-review.md approved explicit-scope local binding index builder pending final CI gate.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt, cargo clippy, cargo test, M0016 data-model/accepted/authority validators, M0015 validator, and M0002 validator passed.
```

## Handoff

- Next main task: `main-task implementation`
- Reason: `Add local binding index builder after tests fail.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `crates/compiler/src/name_resolution.rs`
