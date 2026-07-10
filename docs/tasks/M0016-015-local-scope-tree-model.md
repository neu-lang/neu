# Task: M0016-015 Add local scope tree model

## Task Metadata

- Task ID: `M0016-015`
- Milestone: `M0016`
- Milestone File: `docs/milestones/M0016-name-resolution-pass.md`
- Status: `complete`
- Owner Agent: `Implementer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0016-015-local-scope-tree-model`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Project Rules: `docs/AGENTS.md`
- Agent Prompts:
  - `.codex/agents/test-engineer.md`
  - `.codex/agents/implementer.md`
  - `.codex/agents/reviewer.md`

## Goal

Add a deterministic local lexical scope tree data model.

## Motivation

ADR-0026 says declaration bodies and block expressions introduce lexical scopes. Local binding storage exists, but the compiler still needs a data model that can represent a scope's owner AST node and parent scope before implementing parser-driven scope construction or lookup.

## Scope

- Add a local scope record with id, owner AST node, and optional parent scope.
- Add a local scope tree that preserves insertion order.
- Allocate stable `LocalScopeId` values from insertion order.
- Support retrieving scopes by id and iterating all scopes.

## Out Of Scope

- Parser-driven scope construction.
- Local binding assignment to discovered scopes.
- Local lookup.
- Declaration-order visibility.
- Expression name-reference resolution.
- Function parameter bindings.
- Pattern bindings.
- Import, cross-module, member, overload, extension, or type-directed lookup.

## Required Inputs

- Accepted ADR: `docs/adr/ADR-0026-name-resolution-policy.md`
- AST node ids from `crates/newlang/src/ast.rs`
- Existing local binding model from `crates/newlang/src/name_resolution.rs`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Scope tree allocates stable scope ids in insertion order.
  - Scope records preserve owner AST node and parent scope.
  - Root and child scopes can be retrieved by id.
- Negative tests:
  - Unknown scope ids return `None`.
- Adversarial tests:
  - Scope tree does not implement lookup.
  - Scope tree does not construct scopes from parser structure.

## Test-First Gate

- Test files to update before implementation:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Local scope tree APIs do not exist.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add `LocalScope` and `LocalScopeTree` storage types beside the local binding model. Keep this as a pure data model with no parser traversal and no lookup algorithm.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing change.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] Reviewer compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.

## Execution Commands

- Generate tests: `update crates/newlang/tests/name_resolution.rs && update docs/tests/m0016-name-resolution-data-model.sh`
- Verify tests fail: `cargo test -p newlang --test name_resolution`
- Ordinary tests: `cargo test -p newlang --test name_resolution && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0016-015-local-scope-tree-model.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0016-015-local-scope-tree-model.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh && docs/tests/m0016-name-resolution-blocked.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Implementation files:
  - `crates/newlang/src/name_resolution.rs`
- Documentation or checklist files:
  - `docs/tasks/M0016-015-local-scope-tree-model.md`

## Forbidden Changes

- Do not construct scopes from parser output.
- Do not implement local lookup.
- Do not traverse expression AST.
- Do not assign local bindings to discovered scopes.
- Do not modify accepted ADR-0026.

## Ambiguities And Dependencies

- Parser-driven scope construction remains a later M0016 task.
- Declaration-order visibility remains a later M0016 task.

## Execution Log

```text
2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created M0016 local scope tree model task.
2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Updated name-resolution tests and M0016 data-model validator before adding local scope tree APIs.
2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=cargo test -p newlang --test name_resolution failed before implementation because LocalScopeTree was missing.
2026-07-10 agent=Implementer phase=implementation result=pass notes=Added storage-only LocalScope and LocalScopeTree with stable insertion-order scope ids.
2026-07-10 agent=Test-Engineer phase=ordinary-tests result=pass notes=cargo test -p newlang --test name_resolution, M0016 data-model validator, and M0016 accepted-state validator passed.
2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created a soundness report after ordinary tests were recorded; concrete adversarial review found no parser construction or lookup.
2026-07-10 agent=Reviewer phase=review result=pass notes=docs/tasks/reviews/M0016-015-review.md approved local scope tree model pending final CI gate.
2026-07-10 agent=Build-Engineer phase=ci result=pass notes=cargo fmt, cargo clippy, cargo test, M0016 data-model/accepted/authority validators, M0015 validator, and M0002 validator passed.
```

## Handoff

- Next Agent: `Implementer`
- Reason: `Add local scope tree data model after tests fail.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `crates/newlang/src/name_resolution.rs`
