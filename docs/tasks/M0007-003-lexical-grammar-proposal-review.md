# Task: M0007-003 Review Draft Lexical Grammar Proposal

## Task Metadata

- Task ID: `M0007-003`
- Milestone: `M0007`
- Milestone File: `docs/milestones/M0007-lexer-implementation.md`
- Status: `complete`
- Owner Agent: `Reviewer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-09`
- Branch: `task/M0007-003-lexical-grammar-proposal-review`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/`
- Project Rules: `docs/AGENTS.md`
- Agent Prompts:
  - `.codex/agents/reviewer.md`
  - `.codex/agents/adversarial-engineer.md`
  - `.codex/agents/diagnostics-engineer.md`
  - `.codex/agents/simplicity-guardian.md`
  - `.codex/agents/chief-architect.md`

## Goal

Review the draft lexical grammar ADR proposal and record the review findings required before it can be accepted as source of truth.

## Motivation

M0007 lexer implementation remains blocked because the project has no accepted lexical grammar. The draft proposal must be reviewed for soundness, diagnostic quality, and simplicity before the Chief Architect can decide whether to accept it or request revisions.

## Scope

- Add review artifacts for the draft lexical grammar proposal.
- Add validation that required reviews exist and keep the proposal non-authoritative.
- Record a pending Chief Architect decision.
- Keep the lexical grammar ambiguity open.
- Keep M0007 implementation blocked until accepted source of truth exists.

## Out Of Scope

- Modifying `docs/SPEC.md`.
- Moving the proposal into accepted `docs/adr/`.
- Resolving `docs/ambiguities/M0006-lexical-grammar.md`.
- Implementing lexer code.
- Adding concrete lexer source-text fixtures.
- Defining accepted language semantics.

## Required Inputs

- Milestone: `docs/milestones/M0007-lexer-implementation.md`
- Spec sections:
  - `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- Existing files:
  - `docs/adr/proposals/ADR-0021-lexical-grammar.md`
  - `docs/ambiguities/M0006-lexical-grammar.md`
  - `docs/lexer/token-model.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - `docs/tests/m0007-lexical-grammar-review.sh` verifies all required review artifacts exist.
- Negative tests:
  - The validation script must fail before review artifacts are added.
- Diagnostic tests:
  - Diagnostics review must cite lexical error categories, source spans, and ADR-0015.
- Adversarial tests:
  - Confirm the Chief Architect decision remains pending.
  - Confirm the proposal remains draft and non-authoritative.
  - Confirm the ambiguity report remains open.
  - Confirm no lexer implementation or concrete lexer fixtures are introduced.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0007-lexical-grammar-review.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Required proposal review artifacts do not exist yet.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add review documents under `docs/adr/proposals/reviews/` for adversarial, diagnostics, simplicity, and Chief Architect decision tracking. The Chief Architect decision must remain pending and must not accept the draft as source of truth.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing change.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] Reviewer compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [x] Milestone checklist is not marked complete because M0007 remains blocked.

## Execution Commands

- Generate tests: `create docs/tests/m0007-lexical-grammar-review.sh`
- Verify tests fail: `docs/tests/m0007-lexical-grammar-review.sh`
- Ordinary tests: `docs/tests/m0007-lexical-grammar-review.sh && docs/tests/m0007-lexical-grammar-proposal.sh && docs/tests/m0007-lexer-blocked.sh`
- Adversarial tests: `docs/tests/m0007-lexical-grammar-review.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0007-003-lexical-grammar-proposal-review.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0007-lexical-grammar-review.sh && docs/tests/m0007-lexical-grammar-proposal.sh && docs/tests/m0007-lexer-blocked.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0007-lexical-grammar-review.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/tasks/M0007-003-lexical-grammar-proposal-review.md`
  - `docs/adr/proposals/reviews/ADR-0021-adversarial-review.md`
  - `docs/adr/proposals/reviews/ADR-0021-diagnostics-review.md`
  - `docs/adr/proposals/reviews/ADR-0021-simplicity-review.md`
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
- Chief Architect approval is required before the proposal can become accepted source of truth.
- Language Designer ownership review is still required before acceptance.

## Execution Log

Append entries as the task progresses.

```text
2026-07-09 agent=Task-Decomposer phase=create-task result=pass notes=Created M0007 lexical grammar proposal review task.
2026-07-09 agent=Test-Engineer phase=generate-tests result=pass notes=Created docs/tests/m0007-lexical-grammar-review.sh before adding review artifacts.
2026-07-09 agent=Test-Engineer phase=verify-tests-fail result=pass notes=Validation failed as expected: missing docs/adr/proposals/reviews/ADR-0021-adversarial-review.md.
2026-07-09 agent=Reviewer phase=ordinary-tests result=pass notes=M0007 review validation passed after review artifacts were added.
2026-07-09 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=Validation confirms proposal remains non-authoritative, ambiguity remains open, and no lexer code or concrete fixtures were added.
2026-07-09 agent=Reviewer phase=review result=pass notes=Review package records required blockers without accepting lexical semantics.
2026-07-09 agent=Build-Engineer phase=ci result=pass notes=Full CI-equivalent gate passed.
```

## Handoff

- Next Agent: `Reviewer`
- Reason: `Review the draft lexical grammar proposal without accepting it as source of truth.`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - `docs/adr/proposals/ADR-0021-lexical-grammar.md`
  - `docs/ambiguities/M0006-lexical-grammar.md`
  - `docs/milestones/M0007-lexer-implementation.md`
