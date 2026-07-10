# Task: M0016-007 Add top-level declaration index

## Task Metadata

- Task ID: `M0016-007`
- Milestone: `M0016`
- Milestone File: `docs/milestones/M0016-name-resolution-pass.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0016-007-top-level-declaration-index`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Add the storage model for same-module top-level declarations keyed by ADR-0026's accepted tuple: module, package namespace, declaration name, and declaration kind.

## Motivation

ADR-0026 defines the top-level declaration key needed before lookup can be implemented. A declaration index gives later collection and lookup tasks a stable, duplicate-preserving storage target without parsing or resolving names yet.

## Scope

- Add `DeclarationKind`, `DeclarationKey`, `DeclaredName`, `DeclarationInsert`, and `DeclarationIndex` to `crates/newlang/src/name_resolution.rs`.
- Cover function declarations and type declarations as the accepted bootstrap declaration kinds.
- Preserve insertion order.
- Reject duplicate top-level declaration keys without replacing the existing declaration.
- Add tests for module/package/name/kind key behavior and duplicate preservation.
- Extend the M0016 data-model validator.

## Out Of Scope

- Extracting declaration names from parser output.
- Implementing local binding storage.
- Implementing lexical scopes.
- Implementing lookup or resolution.
- Implementing duplicate diagnostics.
- Implementing parser integration or fixtures.
- Activating imports or cross-module lookup.

## Required Inputs

- Accepted ADR: `docs/adr/ADR-0026-name-resolution-policy.md`
- Module model: `crates/newlang/src/module.rs`
- Symbol IDs: `crates/newlang/src/symbol.rs`
- AST IDs: `crates/newlang/src/ast.rs`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Declaration keys preserve module, package namespace, symbol ID, and declaration kind.
  - Declaration index preserves insertion order.
  - The same name may exist in different packages, modules, or declaration kinds.
- Negative tests:
  - Duplicate declaration keys preserve the existing declaration and report the attempted duplicate.
  - The implementation does not add lookup, scope stack, import resolver, or parser integration.
- Diagnostic tests:
  - Duplicate detection provides the data needed for later `duplicate_name` diagnostics.
- Adversarial tests:
  - Declaration kind participates in the key so a later task cannot accidentally collapse function and type declarations.

## Test-First Gate

- Test files to update before implementation:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Declaration index types do not exist.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add top-level declaration storage records only. Keep collection, lookup, local bindings, and diagnostics emission for later tasks.

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
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0016-007-top-level-declaration-index.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0016-007-top-level-declaration-index.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh && docs/tests/m0016-name-resolution-blocked.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Implementation files:
  - `crates/newlang/src/name_resolution.rs`
- Documentation or checklist files:
  - `docs/tasks/M0016-007-top-level-declaration-index.md`

## Forbidden Changes

- Do not implement lookup.
- Do not add scope stack behavior.
- Do not add import resolver behavior.
- Do not add parser integration.
- Do not implement duplicate diagnostics.
- Do not modify accepted ADR-0026.

## Ambiguities And Dependencies

- Identifier extraction from parser output remains a later task because current AST nodes do not preserve declaration name text.

## Execution Log

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0016 top-level declaration index task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Updated Rust and docs validators before adding declaration index types.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=cargo test -p newlang --test name_resolution failed before implementation because declaration index APIs were missing.
2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Added DeclarationKind, DeclarationKey, DeclaredName, DeclarationInsert, and DeclarationIndex without collection or lookup behavior.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=cargo test -p newlang --test name_resolution, M0016 data-model validator, and M0016 accepted-state validator passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0016-007-soundness.md after ordinary tests were recorded.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0016-007-review.md approved declaration-index-only scope pending final CI gate.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt, cargo clippy, cargo test, M0016 data-model/accepted/authority validators, M0015 validator, and M0002 validator passed.
```

## Handoff

- Next main task: `main-task implementation`
- Reason: `Add declaration index data model after tests fail.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `crates/newlang/src/name_resolution.rs`
