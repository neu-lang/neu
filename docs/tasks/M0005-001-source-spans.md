# Task: M0005-001 Create Source Database And Span Mapping

## Task Metadata

- Task ID: `M0005-001`
- Milestone: `M0005`
- Milestone File: `docs/milestones/M0005-source-database-spans-and-file-identity.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-09`
- Branch: `task/M0005-001-source-spans`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Implement minimal source-file identity, text storage, byte spans, and ASCII line/column mapping needed by later diagnostics.

## Motivation

M0005 requires stable file identifiers and span mapping before lexing, parsing, or semantic diagnostics can be implemented.

## Scope

- Add a source database module.
- Add stable per-database source file identifiers.
- Add byte span representation with checked construction.
- Add ASCII line and column mapping tests for empty, single-line, multi-line, and invalid span cases.
- Record Unicode column semantics as deferred until specified.

## Out Of Scope

- Lexer, parser, AST, HIR, MIR, semantic analysis, or backend behavior.
- Module resolution.
- Unicode display-width policy.
- Diagnostic rendering.

## Required Inputs

- Milestone: `docs/milestones/M0005-source-database-spans-and-file-identity.md`
- Spec sections:
  - M0005 milestone acceptance criteria.
  - `docs/diagnostics.md`
- ADRs:
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- Existing files:
  - `crates/compiler/src/lib.rs`
  - `docs/diagnostics.md`
  - `docs/test-harness.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Unit tests prove stable file IDs, empty file mapping, single-line mapping, and multi-line mapping for ASCII input.
- Negative tests:
  - Unit tests prove invalid spans and offsets outside a file are rejected.
- Diagnostic tests:
  - Not applicable; source mapping supports later diagnostics but emits none.
- Adversarial tests:
  - Confirm no lexer/parser/semantic/backend files are introduced.

## Test-First Gate

- Test files to create before implementation:
  - `crates/compiler/src/source.rs` tests, before implementation body is added.
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Source database and span types are not implemented yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add the smallest source module that stores files in insertion order, returns stable file IDs, validates byte spans against file length, and maps ASCII byte offsets to 1-based line and column positions.

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

- Generate tests: `add source module tests before implementation`
- Verify tests fail: `cargo test --workspace --all-targets source::tests`
- Ordinary tests: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`
- Adversarial tests: `docs/tests/m0005-source-spans.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0005-001-source-spans.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/compiler/src/source.rs`
  - `docs/tests/m0005-source-spans.sh`
- Implementation files:
  - `crates/compiler/src/lib.rs`
  - `crates/compiler/src/source.rs`
- Documentation or checklist files:
  - `docs/milestones/M0005-source-database-spans-and-file-identity.md`
  - `docs/tasks/M0005-001-source-spans.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not weaken or delete failing tests without main-task review approval.
- Do not implement work outside this task scope.
- Do not introduce language semantics not present in `docs/SPEC.md` or `docs/adr/`.

## Ambiguities And Dependencies

- Unicode column semantics are unresolved. This task implements and tests ASCII byte-column mapping only, and records Unicode display-width behavior as deferred.
- Source encoding policy beyond valid Rust `String` text is deferred.

## Execution Log

Append entries as the task progresses.

```text
2026-07-09 main_task=Task-Decomposer phase=create-task result=pass notes=Created first M0005 task for source database and span mapping.
2026-07-09 main_task=main-task test work phase=generate-tests result=pass notes=Created source module tests and docs/tests/m0005-source-spans.sh before implementation.
2026-07-09 main_task=main-task test work phase=verify-tests-fail result=pass notes=cargo test source::tests failed as expected because SourceDatabase, SourceFileId, and LineColumn were missing.
2026-07-09 main_task=main-task implementation phase=ordinary-tests result=pass notes=Focused source tests and full ordinary M0005 gate chain passed.
2026-07-09 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=Soundness report approved; invalid spans and out-of-scope compiler files checked.
2026-07-09 main_task=main-task review phase=review result=pass notes=Review approved against docs/SPEC.md and M0005.
2026-07-09 main_task=main-task implementation phase=ci result=pass notes=Final CI-equivalent M0005 gate chain passed.
```

## Handoff

- Next main task: `main-task roadmap planning`
- Reason: `M0005 is complete; select M0006 next.`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - `docs/diagnostics.md`
  - `docs/milestones/M0005-source-database-spans-and-file-identity.md`
