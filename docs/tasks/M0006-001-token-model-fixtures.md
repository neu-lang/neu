# Task: M0006-001 Define Token Model Planning And Lexer Fixture Metadata

## Task Metadata

- Task ID: `M0006-001`
- Milestone: `M0006`
- Milestone File: `docs/milestones/M0006-token-model-and-lexer-fixtures.md`
- Status: `complete`
- Owner main task: `main-task language review`
- Created By: `main-task task planning`
- Created Date: `2026-07-09`
- Branch: `task/M0006-001-token-model-fixtures`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Define token-model planning artifacts and lexer fixture metadata without implementing lexing or inventing lexical grammar.

## Motivation

M0006 requires the project to identify what token categories are authorized by `docs/SPEC.md` and ADRs, and to record missing lexical grammar as a blocker for future lexer implementation.

## Scope

- Add token-model planning documentation.
- Classify token categories as specified, blocked, or deferred.
- Add inert lexer fixture metadata that cites `docs/SPEC.md` and M0006.
- Add an ambiguity report for missing detailed lexical grammar.
- Add a validation script for M0006 artifacts.

## Out Of Scope

- Lexer implementation.
- Token enum or compiler source code.
- Parser work.
- New syntax decisions.
- Kotlin grammar assumptions not backed by `docs/SPEC.md` or ADRs.

## Required Inputs

- Milestone: `docs/milestones/M0006-token-model-and-lexer-fixtures.md`
- Spec sections:
  - `docs/SPEC.md`
- ADRs:
  - `docs/adr/`
- Existing files:
  - `docs/test-harness.md`
  - `tests/fixtures/`
  - `main task rules`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - `docs/tests/m0006-token-model-fixtures.sh` verifies token planning docs, fixture metadata, and ambiguity report exist.
- Negative tests:
  - The validation script must fail before implementation because M0006 artifacts are absent.
- Diagnostic tests:
  - Not applicable; this task emits no compiler diagnostics.
- Adversarial tests:
  - Confirm no lexer implementation or token enum is introduced.
  - Confirm lexer fixtures do not include source text or expected tokens.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0006-token-model-fixtures.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Token model planning docs, inert lexer fixture metadata, and ambiguity report do not exist yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add documentation and metadata only: `docs/lexer/token-model.md`, one inert lexer fixture, and one ambiguity report recording that detailed lexical grammar is missing.

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

- Generate tests: `create docs/tests/m0006-token-model-fixtures.sh`
- Verify tests fail: `docs/tests/m0006-token-model-fixtures.sh`
- Ordinary tests: `docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`
- Adversarial tests: `docs/tests/m0006-token-model-fixtures.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0006-001-token-model-fixtures.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0006-token-model-fixtures.sh`
- Implementation files:
  - `docs/lexer/token-model.md`
  - `tests/fixtures/lexer/M0006-inert.fixture.toml`
  - `docs/ambiguities/M0006-lexical-grammar.md`
- Documentation or checklist files:
  - `docs/milestones/M0006-token-model-and-lexer-fixtures.md`
  - `docs/tasks/M0006-001-token-model-fixtures.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not weaken or delete failing tests without main-task review approval.
- Do not implement lexer code.
- Do not add token enums.
- Do not introduce language syntax or lexical semantics not present in `docs/SPEC.md` or `docs/adr/`.

## Ambiguities And Dependencies

- Detailed lexical grammar is missing from `docs/SPEC.md`.
- This task must record that missing grammar as a dependency and must not create source-text lexer fixtures.

## Execution Log

Append entries as the task progresses.

```text
2026-07-09 main_task=Task-Decomposer phase=create-task result=pass notes=Created first M0006 task for token model planning and lexer fixture metadata.
2026-07-09 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0006-token-model-fixtures.sh before implementation.
2026-07-09 main_task=main-task test work phase=verify-tests-fail result=pass notes=Validation failed as expected: missing docs/lexer/token-model.md.
2026-07-09 main_task=Language-Lawyer phase=ordinary-tests result=pass notes=M0006 token model fixture validation and prior milestone gates passed.
2026-07-09 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=Soundness report approved; no lexical behavior or token implementation introduced.
2026-07-09 main_task=main-task review phase=review result=pass notes=Review approved against docs/SPEC.md and M0006.
2026-07-09 main_task=Language-Lawyer phase=ci result=pass notes=Final CI-equivalent M0006 gate and prior milestone gates passed.
```

## Handoff

- Next main task: `main-task roadmap planning`
- Reason: `M0006 is complete; M0007 remains blocked until lexical grammar ambiguity is resolved.`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - `docs/milestones/M0006-token-model-and-lexer-fixtures.md`
