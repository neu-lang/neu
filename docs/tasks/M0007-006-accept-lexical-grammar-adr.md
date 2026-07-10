# Task: M0007-006 Accept Concrete Lexical Grammar ADR

## Task Metadata

- Task ID: `M0007-006`
- Milestone: `M0007`
- Milestone File: `docs/milestones/M0007-lexer-implementation.md`
- Status: `complete`
- Owner main task: `main task`
- Created By: `main-task task planning`
- Created Date: `2026-07-09`
- Branch: `task/M0007-006-accept-lexical-grammar-adr`

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
  - `main task rules`
  - `main task rules`

## Goal

Accept a concrete lexical grammar ADR as source of truth so M0007 can proceed to fixture and lexer implementation tasks without guessing syntax.

## Motivation

M0007 is blocked because no accepted source-of-truth document defines whitespace, comments, identifiers, keywords, literals, operators, delimiters, lexical errors, or lexical diagnostic spans. The draft proposal and required reviews are complete, so the next step is a main task decision that accepts concrete lexical rules.

## Scope

- Add accepted `docs/adr/ADR-0021-lexical-grammar.md`.
- Add an ADR-0021 summary to `docs/SPEC.md`.
- Update `docs/ambiguities/M0006-lexical-grammar.md` to resolved.
- Update the main task decision document to approved.
- Replace open-blocker validation with resolved-state validation.
- Preserve old proposal and review artifacts as historical context.

## Out Of Scope

- Implementing lexer code.
- Adding concrete lexer source fixtures.
- Parser grammar, precedence, or recovery behavior.
- Type semantics of literals beyond lexical classification and overflow responsibility.
- Unicode identifier acceptance beyond explicit deferral.

## Required Inputs

- Milestone: `docs/milestones/M0007-lexer-implementation.md`
- Spec sections:
  - `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- Existing files:
  - `docs/adr/proposals/ADR-0021-lexical-grammar.md`
  - `docs/adr/proposals/reviews/ADR-0021-language-designer-review.md`
  - `docs/adr/proposals/reviews/ADR-0021-adversarial-review.md`
  - `docs/adr/proposals/reviews/ADR-0021-diagnostics-review.md`
  - `docs/adr/proposals/reviews/ADR-0021-simplicity-review.md`
  - `docs/adr/proposals/reviews/ADR-0021-chief-architect-decision.md`
  - `docs/ambiguities/M0006-lexical-grammar.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - `docs/tests/m0007-lexical-grammar-accepted.sh` verifies ADR-0021 exists as accepted source of truth and contains required lexical decisions.
- Negative tests:
  - The validation script must fail before accepted ADR-0021 and resolved ambiguity updates exist.
- Diagnostic tests:
  - The accepted ADR must define lexical error categories and source span rules.
- Adversarial tests:
  - Confirm no lexer implementation is introduced.
  - Confirm no concrete lexer fixtures are introduced before the next M0007 task.
  - Confirm the former blocker is resolved only because accepted source of truth exists.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0007-lexical-grammar-accepted.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Accepted `docs/adr/ADR-0021-lexical-grammar.md` does not exist yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Promote the draft direction into an accepted ADR by defining a small Kotlin-like custom lexical grammar. Update blocker tracking documents to show the ambiguity resolved by accepted ADR-0021. Supersede old blocker-status tests with accepted-grammar validation because M0007 is no longer blocked for lack of lexical grammar.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing source-of-truth change.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] main-task review compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [x] M0007 implementation remains incomplete because lexer code and fixtures are separate tasks.

## Execution Commands

- Generate tests: `create docs/tests/m0007-lexical-grammar-accepted.sh`
- Verify tests fail: `docs/tests/m0007-lexical-grammar-accepted.sh`
- Ordinary tests: `docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0007-language-designer-review.sh && docs/tests/m0007-lexical-grammar-review.sh && docs/tests/m0007-lexical-grammar-proposal.sh`
- Adversarial tests: `docs/tests/m0007-lexical-grammar-accepted.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0007-006-accept-lexical-grammar-adr.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0007-language-designer-review.sh && docs/tests/m0007-lexical-grammar-review.sh && docs/tests/m0007-lexical-grammar-proposal.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0007-lexical-grammar-accepted.sh`
  - `docs/tests/m0007-lexer-blocked.sh`
  - `docs/tests/m0007-blocker-status-sync.sh`
  - `docs/tests/m0007-lexical-grammar-review.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/tasks/M0007-006-accept-lexical-grammar-adr.md`
  - `docs/adr/ADR-0021-lexical-grammar.md`
  - `docs/SPEC.md`
  - `docs/ambiguities/M0006-lexical-grammar.md`
  - `docs/adr/proposals/reviews/ADR-0021-chief-architect-decision.md`

## Forbidden Changes

- Do not implement lexer code.
- Do not add concrete lexer fixtures.
- Do not modify unrelated ADRs.
- Do not define parser grammar or precedence.
- Do not define type checking rules for literals beyond lexical overflow responsibility.

## Ambiguities And Dependencies

- Parser grammar remains outside this task.
- Unicode identifier support is explicitly deferred by accepted ADR-0021.
- Lexer implementation remains a later M0007 task.

## Execution Log

Append entries as the task progresses.

```text
2026-07-09 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0007 accepted lexical grammar ADR task.
2026-07-09 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0007-lexical-grammar-accepted.sh before adding accepted ADR-0021.
2026-07-09 main_task=main-task test work phase=verify-tests-fail result=pass notes=Validation failed as expected: missing docs/adr/ADR-0021-lexical-grammar.md.
2026-07-09 main_task=Chief-Architect phase=ordinary-tests result=pass notes=Accepted grammar and resolved-state M0007 validators passed after ADR-0021 was accepted and blocker docs were updated.
2026-07-09 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=Validation confirms acceptance did not add lexer code or concrete fixtures and diagnostics are specified in ADR-0021.
2026-07-09 main_task=main-task review phase=review result=pass notes=Review approved accepted lexical grammar ADR as source-of-truth update for M0007.
2026-07-09 main_task=Build-Engineer phase=ci result=pass notes=Full CI-equivalent gate passed.
```

## Handoff

- Next main task: `main task`
- Reason: `Accept or reject concrete lexical grammar as source of truth.`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - `docs/adr/proposals/ADR-0021-lexical-grammar.md`
  - `docs/adr/proposals/reviews/`
  - `docs/ambiguities/M0006-lexical-grammar.md`
