# Task: M0016-011 Add top-level lookup query API

## Task Metadata

- Task ID: `M0016-011`
- Milestone: `M0016`
- Milestone File: `docs/milestones/M0016-name-resolution-pass.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0016-011-top-level-lookup-query`

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

Add an explicit same-module package top-level declaration lookup query over `DeclarationIndex`.

## Motivation

ADR-0026 requires lookup from the current package namespace in the current module after local scopes. Local scopes are not implemented yet, but the top-level declaration index can support an explicit query API that later lookup orchestration can call.

## Scope

- Add `TopLevelLookup`, `TopLevelLookupResult`, and `DeclarationIndex::lookup_top_level`.
- Return a found declaration when the exact module, package namespace, name symbol, and declaration kind exists.
- Return `unresolved_name` diagnostic data when the key is absent.
- Preserve exact package and declaration-kind matching.

## Out Of Scope

- Local lexical scopes.
- Expression AST traversal.
- Full name-resolution pass orchestration.
- Ambiguous-name diagnostics.
- Imports.
- Cross-module lookup.
- Member lookup.
- Type-directed lookup.

## Required Inputs

- Accepted ADR: `docs/adr/ADR-0026-name-resolution-policy.md`
- Declaration index from `crates/compiler/src/name_resolution.rs`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Exact top-level query finds a declaration.
  - Same name in a different package is not returned.
  - Same name with a different declaration kind is not returned.
- Negative tests:
  - Missing names return `Unresolved` with `unresolved_name` diagnostic and primary span.
- Adversarial tests:
  - Query does not inspect imports, cross-module state, members, or local scopes.

## Test-First Gate

- Test files to update before implementation:
  - `crates/compiler/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Top-level lookup query API does not exist.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a small explicit query type and result enum over existing declaration index storage. Keep it deterministic and exact-key only.

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
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0016-011-top-level-lookup-query.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0016-011-top-level-lookup-query.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh && docs/tests/m0016-name-resolution-blocked.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/compiler/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Implementation files:
  - `crates/compiler/src/name_resolution.rs`
- Documentation or checklist files:
  - `docs/tasks/M0016-011-top-level-lookup-query.md`

## Forbidden Changes

- Do not implement local scope lookup.
- Do not traverse expression AST.
- Do not activate imports.
- Do not add cross-module lookup.
- Do not add member lookup.
- Do not modify accepted ADR-0026.

## Ambiguities And Dependencies

- Local lexical lookup remains a later task.

## Execution Log

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0016 top-level lookup query task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Updated name-resolution and docs validators before adding top-level lookup query API.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=cargo test -p compiler --test name_resolution failed before implementation because TopLevelLookup APIs were missing.
2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Added exact-key same-module package top-level lookup query returning found declarations or unresolved_name diagnostics.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=cargo test -p compiler --test name_resolution, M0016 data-model validator, and M0016 accepted-state validator passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0016-011-soundness.md after ordinary tests were recorded.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0016-011-review.md approved exact top-level lookup query scope pending final CI gate.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt, cargo clippy, cargo test, M0016 data-model/accepted/authority validators, M0015 validator, and M0002 validator passed.
```

## Handoff

- Next main task: `main-task implementation`
- Reason: `Add top-level lookup query API after tests fail.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `crates/compiler/src/name_resolution.rs`
