# Task: M0007-005 Synchronize Lexical Grammar Blocker Review Status

## Task Metadata

- Task ID: `M0007-005`
- Milestone: `M0007`
- Milestone File: `docs/milestones/M0007-lexer-implementation.md`
- Status: `complete`
- Owner main task: `main-task language review`
- Created By: `main-task task planning`
- Created Date: `2026-07-09`
- Branch: `task/M0007-005-lexical-grammar-blocker-status-sync`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`

## Goal

Synchronize lexical grammar blocker tracking documents with completed review artifacts while keeping the blocker open.

## Motivation

M0007 cannot proceed until lexical grammar is accepted. Several required reviews now exist, but the ambiguity report and main task decision still read as if no review progress has occurred.

## Scope

- Update `docs/ambiguities/M0006-lexical-grammar.md` checklist entries for completed review artifacts.
- Update `docs/adr/proposals/reviews/ADR-0021-chief-architect-decision.md` to distinguish completed reviews from remaining acceptance blockers.
- Add validation that the blocker remains open and the proposal remains non-authoritative.

## Out Of Scope

- Modifying `docs/SPEC.md`.
- Moving the proposal into accepted `docs/adr/`.
- Resolving the ambiguity report.
- Accepting lexical semantics.
- Implementing lexer code.
- Adding concrete lexer fixtures.

## Required Inputs

- Milestone: `docs/milestones/M0007-lexer-implementation.md`
- Spec sections:
  - `docs/SPEC.md`
- ADRs:
  - `docs/adr/`
- Existing files:
  - `docs/ambiguities/M0006-lexical-grammar.md`
  - `docs/adr/proposals/ADR-0021-lexical-grammar.md`
  - `docs/adr/proposals/reviews/ADR-0021-language-designer-review.md`
  - `docs/adr/proposals/reviews/ADR-0021-adversarial-review.md`
  - `docs/adr/proposals/reviews/ADR-0021-diagnostics-review.md`
  - `docs/adr/proposals/reviews/ADR-0021-simplicity-review.md`
  - `docs/adr/proposals/reviews/ADR-0021-chief-architect-decision.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - `docs/tests/m0007-blocker-status-sync.sh` verifies completed review steps are marked and remaining semantic blockers are explicit.
- Negative tests:
  - The validation script must fail before tracking documents are synchronized.
- Diagnostic tests:
  - Not applicable beyond preserving diagnostics review status.
- Adversarial tests:
  - Confirm the ambiguity remains open.
  - Confirm the main task decision remains pending.
  - Confirm no lexer implementation or concrete lexer fixtures are introduced.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0007-blocker-status-sync.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Blocker tracking documents have not been synchronized with completed reviews.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Update only status-tracking documentation to reflect completed review artifacts. Do not change source-of-truth language semantics.

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
- [x] Milestone checklist is not marked complete because M0007 remains blocked.

## Execution Commands

- Generate tests: `create docs/tests/m0007-blocker-status-sync.sh`
- Verify tests fail: `docs/tests/m0007-blocker-status-sync.sh`
- Ordinary tests: `docs/tests/m0007-blocker-status-sync.sh && docs/tests/m0007-language-designer-review.sh && docs/tests/m0007-lexical-grammar-review.sh && docs/tests/m0007-lexer-blocked.sh`
- Adversarial tests: `docs/tests/m0007-blocker-status-sync.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0007-005-lexical-grammar-blocker-status-sync.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0007-blocker-status-sync.sh && docs/tests/m0007-language-designer-review.sh && docs/tests/m0007-lexical-grammar-review.sh && docs/tests/m0007-lexical-grammar-proposal.sh && docs/tests/m0007-lexer-blocked.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0007-blocker-status-sync.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/tasks/M0007-005-lexical-grammar-blocker-status-sync.md`
  - `docs/ambiguities/M0006-lexical-grammar.md`
  - `docs/adr/proposals/reviews/ADR-0021-chief-architect-decision.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted files under `docs/adr/`.
- Do not close `docs/ambiguities/M0006-lexical-grammar.md`.
- Do not implement lexer code.
- Do not add concrete lexer fixtures.
- Do not treat the draft proposal as accepted semantics.

## Ambiguities And Dependencies

- Blocking ambiguity remains `docs/ambiguities/M0006-lexical-grammar.md`.
- main task approval is still required before the proposal can become accepted source of truth.
- A concrete lexical grammar is still missing from source of truth.

## Execution Log

Append entries as the task progresses.

```text
2026-07-09 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0007 blocker status synchronization task.
2026-07-09 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0007-blocker-status-sync.sh before synchronizing blocker documents.
2026-07-09 main_task=main-task test work phase=verify-tests-fail result=pass notes=Validation failed as expected because blocker tracking documents were not synchronized with completed reviews.
2026-07-09 main_task=Language-Lawyer phase=ordinary-tests result=pass notes=M0007 blocker status synchronization validation passed after updating tracking documents.
2026-07-09 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=Validation confirms status sync does not accept semantics, close ambiguity, or add lexer code.
2026-07-09 main_task=main-task review phase=review result=pass notes=Review approved status synchronization as scoped governance work.
2026-07-09 main_task=Build-Engineer phase=ci result=pass notes=Full CI-equivalent gate passed.
```

## Handoff

- Next main task: `main-task language review`
- Reason: `Synchronize blocker status without resolving the ambiguity.`
- Required Context:
  - This task file
  - `docs/ambiguities/M0006-lexical-grammar.md`
  - `docs/adr/proposals/reviews/ADR-0021-chief-architect-decision.md`
