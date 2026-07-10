# Task: M0007-004 main-task semantic design Ownership Review For Draft Lexical Grammar

## Task Metadata

- Task ID: `M0007-004`
- Milestone: `M0007`
- Milestone File: `docs/milestones/M0007-lexer-implementation.md`
- Status: `complete`
- Owner main task: `main-task semantic design`
- Created By: `main-task task planning`
- Created Date: `2026-07-09`
- Branch: `task/M0007-004-lexical-grammar-language-designer-review`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Record main-task semantic design ownership review for the draft lexical grammar proposal and identify the concrete semantic content still required before acceptance.

## Motivation

The main task pending decision for ADR-0021 lists main-task semantic design ownership review as required before acceptance. Completing that review advances M0007's blocker resolution path without accepting semantics or implementing the lexer.

## Scope

- Add main-task semantic design review for `docs/adr/proposals/ADR-0021-lexical-grammar.md`.
- Confirm the proposal direction is compatible with Kotlin-like ergonomics.
- Identify required semantic specifics that must be added before acceptance.
- Keep the proposal non-authoritative.
- Keep the lexical grammar ambiguity open.

## Out Of Scope

- Modifying `docs/SPEC.md`.
- Moving the proposal into accepted `docs/adr/`.
- Resolving `docs/ambiguities/M0006-lexical-grammar.md`.
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
  - `docs/adr/proposals/ADR-0021-lexical-grammar.md`
  - `docs/adr/proposals/reviews/ADR-0021-chief-architect-decision.md`
  - `docs/ambiguities/M0006-lexical-grammar.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - `docs/tests/m0007-language-designer-review.sh` verifies the main-task semantic design review exists and records required acceptance content.
- Negative tests:
  - The validation script must fail before the review artifact is added.
- Diagnostic tests:
  - The review must require diagnostic-facing lexical rules before acceptance.
- Adversarial tests:
  - Confirm the review does not accept the proposal as source of truth.
  - Confirm the ambiguity remains open.
  - Confirm no lexer implementation or concrete lexer fixtures are introduced.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0007-language-designer-review.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - main-task semantic design review artifact does not exist yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add `docs/adr/proposals/reviews/ADR-0021-language-designer-review.md` documenting main-task semantic design ownership review and required revisions before acceptance. Do not update source of truth files.

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

- Generate tests: `create docs/tests/m0007-language-designer-review.sh`
- Verify tests fail: `docs/tests/m0007-language-designer-review.sh`
- Ordinary tests: `docs/tests/m0007-language-designer-review.sh && docs/tests/m0007-lexical-grammar-review.sh && docs/tests/m0007-lexical-grammar-proposal.sh && docs/tests/m0007-lexer-blocked.sh`
- Adversarial tests: `docs/tests/m0007-language-designer-review.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0007-004-lexical-grammar-language-designer-review.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0007-language-designer-review.sh && docs/tests/m0007-lexical-grammar-review.sh && docs/tests/m0007-lexical-grammar-proposal.sh && docs/tests/m0007-lexer-blocked.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0007-language-designer-review.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/tasks/M0007-004-lexical-grammar-language-designer-review.md`
  - `docs/adr/proposals/reviews/ADR-0021-language-designer-review.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted files under `docs/adr/`.
- Do not close `docs/ambiguities/M0006-lexical-grammar.md`.
- Do not implement lexer code.
- Do not add concrete lexer fixtures.
- Do not treat the draft proposal as accepted semantics.

## Ambiguities And Dependencies

- Blocking ambiguity remains `docs/ambiguities/M0006-lexical-grammar.md`.
- main task approval is required before the proposal can become accepted source of truth.
- The review may identify required semantic content, but must not define accepted semantics.

## Execution Log

Append entries as the task progresses.

```text
2026-07-09 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0007 main-task semantic design ownership review task.
2026-07-09 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0007-language-designer-review.sh before adding main-task semantic design review.
2026-07-09 main_task=main-task test work phase=verify-tests-fail result=pass notes=Validation failed as expected: missing docs/adr/proposals/reviews/ADR-0021-language-designer-review.md.
2026-07-09 main_task=Language-Designer phase=ordinary-tests result=pass notes=M0007 main-task semantic design ownership review validation passed after review artifact was added.
2026-07-09 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=Validation confirms ownership review remains non-authoritative, ambiguity remains open, and no lexer code or concrete fixtures were added.
2026-07-09 main_task=main-task review phase=review result=pass notes=Review approved ownership-review artifact as scoped governance progress.
2026-07-09 main_task=Build-Engineer phase=ci result=pass notes=Full CI-equivalent gate passed.
```

## Handoff

- Next main task: `main-task semantic design`
- Reason: `Record ownership review for the draft lexical grammar proposal without accepting semantics.`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - `docs/adr/proposals/ADR-0021-lexical-grammar.md`
  - `docs/adr/proposals/reviews/ADR-0021-chief-architect-decision.md`
  - `docs/ambiguities/M0006-lexical-grammar.md`
