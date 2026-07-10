# Task: M0016-018 Add local binding lookup API

## Task Metadata

- Task ID: `M0016-018`
- Milestone: `M0016`
- Milestone File: `docs/milestones/M0016-name-resolution-pass.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0016-018-local-binding-lookup`

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

Add explicit local binding lookup over an existing local scope tree and local binding index.

## Motivation

ADR-0026 requires local lookup to search from the innermost scope outward, while local bindings are visible only after their declaration statement. M0016 now has scope construction and scoped local binding storage. The next safe step is a direct local lookup API that enforces these local-only rules without walking expression ASTs or falling through to top-level declarations.

## Scope

- Add `LocalNameLookup` and `LocalNameLookupResult`.
- Add local lookup over `LocalBindingIndex`.
- Search from a supplied starting scope outward through parent scopes.
- Return the nearest visible local binding.
- Ignore bindings whose declaration statement ends after the reference span starts.
- Return `unresolved_name` diagnostic when no local binding is visible.

## Out Of Scope

- Expression AST traversal.
- Full name-resolution pass orchestration.
- Top-level fallback after local lookup misses.
- Type name lookup.
- Package-qualified lookup.
- Function parameter bindings.
- Pattern bindings.
- Import, cross-module, member, overload, extension, or type-directed lookup.

## Required Inputs

- Accepted ADR: `docs/adr/ADR-0026-name-resolution-policy.md`
- Local scope tree from `crates/compiler/src/name_resolution.rs`
- Scoped local binding index from `crates/compiler/src/name_resolution.rs`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Local lookup finds a binding after its declaration statement.
  - Inner scope binding shadows outer scope binding when visible.
  - If an inner binding is not yet visible, lookup can continue to a visible outer binding.
- Negative tests:
  - Local lookup does not find a binding before its declaration statement.
  - Missing local name returns `unresolved_name` with the reference span.
- Adversarial tests:
  - Lookup does not inspect top-level declarations.
  - Lookup does not inspect imports, members, overloads, or type-directed candidates.

## Test-First Gate

- Test files to update before implementation:
  - `crates/compiler/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Local lookup APIs do not exist.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a direct lookup query type and result enum. Implement lookup by probing exact local binding keys while walking the parent chain in `LocalScopeTree`.

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
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0016-018-local-binding-lookup.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0016-018-local-binding-lookup.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh && docs/tests/m0016-name-resolution-blocked.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/compiler/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Implementation files:
  - `crates/compiler/src/name_resolution.rs`
- Documentation or checklist files:
  - `docs/tasks/M0016-018-local-binding-lookup.md`

## Forbidden Changes

- Do not traverse expression AST.
- Do not implement full name-resolution pass orchestration.
- Do not add top-level fallback.
- Do not add import, member, overload, extension, or type-directed lookup.
- Do not modify accepted ADR-0026.

## Ambiguities And Dependencies

- Name-reference extraction remains a later task.
- Combining local lookup with top-level lookup remains a later task.

## Execution Log

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0016 local binding lookup task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Updated name-resolution tests and M0016 data-model validator before adding local lookup APIs.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=cargo test -p compiler --test name_resolution failed before implementation because LocalNameLookup, LocalNameLookupResult, and lookup_local were missing.
2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Added local binding lookup from a supplied scope outward with declaration-order visibility.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=cargo test -p compiler --test name_resolution, M0016 data-model validator, and M0016 accepted-state validator passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created a soundness report after ordinary tests were recorded; concrete adversarial review found no top-level fallback or full resolution orchestration.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0016-018-review.md approved local-only lookup pending final CI gate.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt, cargo clippy, cargo test, M0016 data-model/accepted/authority validators, M0015 validator, and M0002 validator passed.
```

## Handoff

- Next main task: `main-task implementation`
- Reason: `Add explicit local binding lookup APIs after tests fail.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `crates/compiler/src/name_resolution.rs`
