# Task: M0016-012 Add local binding storage model

## Task Metadata

- Task ID: `M0016-012`
- Milestone: `M0016`
- Milestone File: `docs/milestones/M0016-name-resolution-pass.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0016-012-local-binding-storage`

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

Add deterministic storage for local `val` and `var` bindings used by the future name-resolution pass.

## Motivation

ADR-0026 requires local `val` and `var` statements to bind in the nearest containing lexical block, reject same-scope duplicates, and allow shadowing across nested scopes. Before implementing lexical scope construction or lookup, the compiler needs a storage model that can represent local scope identity and same-scope binding uniqueness.

## Scope

- Add a local scope identifier type.
- Add local binding key, binding kind, binding record, insertion result, and binding index.
- Preserve insertion order for accepted local bindings.
- Reject duplicate same-scope local binding keys without replacing the existing binding.
- Allow the same name symbol in distinct local scopes.

## Out Of Scope

- Parser extraction of local `val` or `var` binding metadata.
- Lexical scope construction.
- Local name lookup.
- Expression AST traversal.
- Function parameter bindings.
- Pattern bindings.
- Duplicate local binding diagnostics.
- Import, cross-module, member, overload, extension, or type-directed lookup.

## Required Inputs

- Accepted ADR: `docs/adr/ADR-0026-name-resolution-policy.md`
- Existing name-resolution data model: `crates/newlang/src/name_resolution.rs`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Local binding keys preserve scope and symbol.
  - Local binding index preserves insertion order and lookup by key.
  - Same name symbol is allowed in distinct local scopes.
  - Binding kind distinguishes `val` and `var`.
- Negative tests:
  - Duplicate same-scope local binding key preserves the existing binding and does not append the attempted binding.
- Adversarial tests:
  - Storage does not implement lexical scope construction or local lookup.
  - Storage does not introduce parser extraction for local declarations.

## Test-First Gate

- Test files to update before implementation:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Local binding storage APIs do not exist.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add storage-only local binding types beside the existing declaration and resolution table types. Keep the API deterministic and exact-key only.

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
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0016-012-local-binding-storage.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0016-012-local-binding-storage.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh && docs/tests/m0016-name-resolution-blocked.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Implementation files:
  - `crates/newlang/src/name_resolution.rs`
- Documentation or checklist files:
  - `docs/tasks/M0016-012-local-binding-storage.md`

## Forbidden Changes

- Do not implement lexical scope construction.
- Do not implement local lookup.
- Do not traverse expression AST.
- Do not extract parser local binding metadata.
- Do not activate imports.
- Do not add cross-module lookup.
- Do not add member lookup.
- Do not modify accepted ADR-0026.

## Ambiguities And Dependencies

- Parser metadata for local `val` and `var` statements remains a later task.
- Lexical visibility before and after declaration statements remains a later task.

## Execution Log

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0016 local binding storage task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Updated name-resolution unit tests and M0016 data-model validator before adding local binding storage APIs.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=cargo test -p newlang --test name_resolution failed before implementation because LocalBinding storage APIs were missing.
2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Added storage-only LocalScopeId, LocalBindingKey, LocalBindingKind, LocalBindingInsert, and LocalBindingIndex APIs.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=cargo test -p newlang --test name_resolution, M0016 data-model validator, and M0016 accepted-state validator passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created a soundness report after ordinary tests were recorded; concrete adversarial review found no scope expansion.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0016-012-review.md approved local binding storage scope pending final CI gate.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt, cargo clippy, cargo test, M0016 data-model/accepted/authority validators, M0015 validator, and M0002 validator passed.
```

## Handoff

- Next main task: `main-task implementation`
- Reason: `Add local binding storage APIs after tests fail.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `crates/newlang/src/name_resolution.rs`
