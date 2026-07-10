# Task: M0016-016 Build local scope tree from AST scope owners

## Task Metadata

- Task ID: `M0016-016`
- Milestone: `M0016`
- Milestone File: `docs/milestones/M0016-name-resolution-pass.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0016-016-build-local-scope-tree`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Build a local scope tree from AST scope-owner nodes.

## Motivation

M0016-015 added the local scope tree storage model. ADR-0026 defines declaration bodies and block expressions as lexical scope boundaries, and states that nested declaration bodies do not inherit local bindings from enclosing declaration bodies. The next safe step is to construct a deterministic scope tree from existing AST nodes without performing local lookup.

## Scope

- Add `build_local_scope_tree`.
- Create scopes for `DeclarationBody` and `Block` AST nodes.
- Allocate scope ids in source order with outer scopes before nested child scopes.
- Parent nested block scopes to the nearest containing block scope.
- Keep declaration body scopes as roots because nested declaration bodies do not inherit enclosing local bindings.

## Out Of Scope

- Local binding assignment to discovered scopes.
- Local lookup.
- Declaration-order visibility.
- Expression name-reference resolution.
- Function parameter bindings.
- Pattern bindings.
- Import, cross-module, member, overload, extension, or type-directed lookup.

## Required Inputs

- Accepted ADR: `docs/adr/ADR-0026-name-resolution-policy.md`
- AST arena from `crates/compiler/src/ast.rs`
- Existing local scope tree model from `crates/compiler/src/name_resolution.rs`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Builder creates scopes for parser `Block` nodes.
  - Nested block scopes parent to the nearest containing block.
  - Builder creates declaration body scopes as roots.
  - Scope ids are deterministic in source order, not parser insertion order.
- Negative tests:
  - Non-scope-owner AST nodes do not create scopes.
- Adversarial tests:
  - Builder does not assign local bindings to scopes.
  - Builder does not implement lookup.

## Test-First Gate

- Test files to update before implementation:
  - `crates/compiler/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `build_local_scope_tree` does not exist.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Select AST nodes whose kind is `DeclarationBody` or `Block`, sort them by source span so containing scopes are created before contained scopes, then add each scope with the accepted parent rule.

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
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0016-016-build-local-scope-tree.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0016-016-build-local-scope-tree.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh && docs/tests/m0016-name-resolution-blocked.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/compiler/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Implementation files:
  - `crates/compiler/src/name_resolution.rs`
- Documentation or checklist files:
  - `docs/tasks/M0016-016-build-local-scope-tree.md`

## Forbidden Changes

- Do not assign local bindings to discovered scopes.
- Do not implement local lookup.
- Do not traverse expression references for resolution.
- Do not add function parameter or pattern bindings.
- Do not modify accepted ADR-0026.

## Ambiguities And Dependencies

- Binding-to-scope assignment remains a later M0016 task.
- Declaration-order visibility remains a later M0016 task.

## Execution Log

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0016 AST-driven local scope tree builder task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Updated name-resolution tests and M0016 data-model validator before adding build_local_scope_tree.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=cargo test -p compiler --test name_resolution failed before implementation because build_local_scope_tree was missing.
2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Added AST-driven local scope tree builder for Block and DeclarationBody scope-owner nodes.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=cargo test -p compiler --test name_resolution, M0016 data-model validator, and M0016 accepted-state validator passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created a soundness report after ordinary tests were recorded; concrete adversarial review found no binding assignment or lookup.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0016-016-review.md approved AST-driven local scope tree construction pending final CI gate.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt, cargo clippy, cargo test, M0016 data-model/accepted/authority validators, M0015 validator, and M0002 validator passed.
```

## Handoff

- Next main task: `main-task implementation`
- Reason: `Add AST-driven local scope tree construction after tests fail.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `crates/compiler/src/name_resolution.rs`
