# Task: M0015-001 Add symbol interner

## Task Metadata

- Task ID: `M0015-001`
- Milestone: `M0015`
- Milestone File: `docs/milestones/M0015-symbol-interning-and-name-tables.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0015-001-symbol-interner`

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

Add stable symbol interning infrastructure independent of name-resolution policy.

## Motivation

M0015 requires stable symbol identities before name tables and name resolution can be built. The first safe slice is an interner that maps exact textual names to stable IDs within one compilation session.

## Scope

- Add a `symbol` module.
- Add `SymbolId`.
- Add `SymbolInterner`.
- Intern exact textual names to stable IDs.
- Resolve interned IDs back to text.
- Preserve insertion order for deterministic tests.
- Add focused Rust tests and a docs validator.
- Mark M0015 symbol identities stable.

## Out Of Scope

- Name tables.
- Duplicate declaration policy.
- Scopes.
- Import resolution.
- Visibility enforcement.
- Type checking.
- Overload resolution.
- Module-aware symbol tables.
- Parser integration.

## Required Inputs

- `docs/milestones/M0015-symbol-interning-and-name-tables.md`
- `docs/adr/ADR-0010-type-system-shape.md`
- `docs/adr/ADR-0025-module-package-visibility-model.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Interning the same textual name returns the same `SymbolId`.
  - Interning different textual names returns distinct IDs.
  - IDs are stable in insertion order.
  - Interned IDs resolve back to their original exact text.
- Negative tests:
  - Unknown raw IDs do not resolve.
- Adversarial tests:
  - Interner does not define scope, duplicate declarations, imports, visibility enforcement, type checking, overload resolution, or name resolution.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0015-symbol-interner.sh`
  - `crates/newlang/tests/symbol.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `crates/newlang/src/symbol.rs` and `newlang::symbol` do not exist.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Implement a small deterministic interner backed by insertion-ordered storage and a text-to-ID map. Do not validate language identifiers or attach module/scope semantics in this task.

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

- Generate tests: `create docs/tests/m0015-symbol-interner.sh and crates/newlang/tests/symbol.rs`
- Verify tests fail: `docs/tests/m0015-symbol-interner.sh`
- Ordinary tests: `cargo test --workspace --all-targets symbol -- --nocapture && docs/tests/m0015-symbol-interner.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0015-001-symbol-interner.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0015-001-symbol-interner.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0015-symbol-interner.sh && docs/tests/m0014-visibility-metadata.sh && docs/tests/m0014-package-namespace-metadata.sh && docs/tests/m0014-module-identity-model.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0015-symbol-interner.sh`
  - `crates/newlang/tests/symbol.rs`
- Implementation files:
  - `crates/newlang/src/lib.rs`
  - `crates/newlang/src/symbol.rs`
- Documentation or checklist files:
  - `docs/milestones/M0015-symbol-interning-and-name-tables.md`
  - `docs/tasks/M0015-001-symbol-interner.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted ADRs.
- Do not weaken or delete failing tests without main-task review approval.
- Do not implement name tables in this task.
- Do not implement duplicate declaration policy.
- Do not implement scope, imports, visibility enforcement, type checking, overload resolution, or name resolution.
- Do not modify parser behavior.

## Ambiguities And Dependencies

- Duplicate-name behavior remains unspecified for this task and must be handled by a later M0015 task as either accepted behavior or an ambiguity report.
- Name tables depend on this interner and M0014 module metadata.

## Execution Log

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0015 symbol interner task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0015-symbol-interner.sh and Rust symbol tests before implementation.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=docs/tests/m0015-symbol-interner.sh failed before implementation because crates/newlang/src/symbol.rs was missing.
2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Added SymbolId and SymbolInterner with stable exact-text interning and resolution only.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=cargo test --workspace --all-targets symbol -- --nocapture and docs/tests/m0015-symbol-interner.sh passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0015-001-soundness.md after ordinary-test evidence.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0015-001-review.md approves symbol interner scope.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && M0015/M0014/M0002 validation scripts passed.
```

## Handoff

- Next main task: `main-task test work`
- Reason: `Add failing tests for symbol interning.`
- Required Context:
  - This task file
  - `docs/milestones/M0015-symbol-interning-and-name-tables.md`
