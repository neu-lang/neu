# Task: M0015-002 Add name table infrastructure

## Task Metadata

- Task ID: `M0015-002`
- Milestone: `M0015`
- Milestone File: `docs/milestones/M0015-symbol-interning-and-name-tables.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0015-002-name-table-infrastructure`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Add name table infrastructure that can hold declarations per module without implementing name-resolution policy.

## Motivation

M0015 requires name table representation and duplicate-name detection hooks before name resolution can enforce visibility and scope rules. The milestone explicitly defers final resolution semantics, so this task stores exact symbol entries and reports duplicate insertion attempts without deciding language-level duplicate legality.

## Scope

- Add `NameTable`.
- Add `NameTableKey` containing module identity and symbol identity.
- Add `NameTableEntry`.
- Insert entries keyed by module plus symbol.
- Return a duplicate-insertion hook when the same key already exists.
- Allow distinct modules to contain entries with the same textual symbol.
- Add focused tests and a docs validator.
- Mark name tables tested and resolution policy deferred.

## Out Of Scope

- Scope stacks.
- Import resolution.
- Visibility enforcement.
- Type checking.
- Overload resolution.
- Duplicate declaration legality.
- Parser integration.
- Access checking or lookup diagnostics.

## Required Inputs

- `docs/milestones/M0015-symbol-interning-and-name-tables.md`
- `crates/newlang/src/module.rs`
- `crates/newlang/src/symbol.rs`
- `crates/newlang/tests/symbol.rs`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Same textual name can be inserted in distinct modules as distinct table entries.
  - Insertion stores and returns the declaration payload.
  - Lookup by exact module plus symbol returns the inserted entry.
- Negative tests:
  - Lookup for a missing key returns none.
  - Duplicate insertion for the same module plus symbol returns a duplicate hook and preserves the original entry.
- Adversarial tests:
  - Name table does not define imports, scopes, visibility enforcement, overload policy, type checking, or resolution diagnostics.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0015-name-table-infrastructure.sh`
  - name table test additions in `crates/newlang/tests/symbol.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `NameTable`, `NameTableKey`, and `NameTableEntry` do not exist.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Extend `crates/newlang/src/symbol.rs` with a small insertion-ordered name table keyed by `ModuleName` and `SymbolId`. Use an enum result to distinguish inserted entries from duplicate insertion hooks without declaring whether duplicates are language errors.

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
- [x] Milestone checklist is updated.

## Execution Commands

- Generate tests: `create docs/tests/m0015-name-table-infrastructure.sh and update crates/newlang/tests/symbol.rs`
- Verify tests fail: `docs/tests/m0015-name-table-infrastructure.sh`
- Ordinary tests: `cargo test --workspace --all-targets symbol -- --nocapture && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0015-symbol-interner.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0015-002-name-table-infrastructure.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0015-002-name-table-infrastructure.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0015-symbol-interner.sh && docs/tests/m0014-visibility-metadata.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0015-name-table-infrastructure.sh`
  - `crates/newlang/tests/symbol.rs`
- Implementation files:
  - `crates/newlang/src/symbol.rs`
- Documentation or checklist files:
  - `docs/milestones/M0015-symbol-interning-and-name-tables.md`
  - `docs/tasks/M0015-002-name-table-infrastructure.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted ADRs.
- Do not weaken or delete failing tests without main-task review approval.
- Do not implement import resolution.
- Do not implement visibility enforcement.
- Do not implement type checking.
- Do not implement overload resolution.
- Do not decide duplicate declaration legality.
- Do not modify parser behavior.

## Ambiguities And Dependencies

- Duplicate-name legality remains a later language decision. This task only exposes duplicate insertion as data.
- Scope hierarchy and lookup policy belong to later name resolution work.

## Execution Log

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0015 name table infrastructure task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0015-name-table-infrastructure.sh and Rust name table tests before implementation.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=docs/tests/m0015-name-table-infrastructure.sh failed before implementation because NameTable was missing.
2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Added NameTable keyed by module and symbol with duplicate insertion hook, without lookup policy or resolution semantics.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=cargo test --workspace --all-targets symbol -- --nocapture plus M0015 name table and symbol interner validators passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0015-002-soundness.md after ordinary-test evidence.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0015-002-review.md approves name table infrastructure scope.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && M0015/M0014/M0002 validation scripts passed.
```

## Handoff

- Next main task: `main-task test work`
- Reason: `Add failing tests for name table infrastructure.`
- Required Context:
  - This task file
  - `docs/milestones/M0015-symbol-interning-and-name-tables.md`
