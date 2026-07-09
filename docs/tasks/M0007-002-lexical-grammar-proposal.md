# Task: M0007-002 Draft Lexical Grammar ADR Proposal

## Task Metadata

- Task ID: `M0007-002`
- Milestone: `M0007`
- Milestone File: `docs/milestones/M0007-lexer-implementation.md`
- Status: `complete`
- Owner Agent: `Language Designer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-09`
- Branch: `task/M0007-002-lexical-grammar-proposal`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/`
- Project Rules: `docs/AGENTS.md`
- Agent Prompts:
  - `.codex/agents/language-designer.md`
  - `.codex/agents/language-lawyer.md`
  - `.codex/agents/adversarial-engineer.md`
  - `.codex/agents/diagnostics-engineer.md`
  - `.codex/agents/simplicity-guardian.md`

## Goal

Draft a lexical grammar ADR proposal that can be reviewed by Language Designer, Adversarial Engineer, Diagnostics Engineer, Simplicity Guardian, and Chief Architect.

## Motivation

M0007 lexer implementation is blocked by `docs/ambiguities/M0006-lexical-grammar.md`. A proposal is needed before the project can accept concrete lexer fixtures or implementation.

## Scope

- Add a draft ADR proposal under `docs/adr/proposals/`.
- Include question, competing designs, trade-offs, recommended draft, downstream consequences, and dependencies.
- Keep the ambiguity open until approval.
- Add validation that the proposal is clearly marked draft and not accepted source of truth.

## Out Of Scope

- Modifying `docs/SPEC.md`.
- Moving the proposal into accepted `docs/adr/`.
- Resolving the ambiguity report.
- Implementing lexer code.
- Adding concrete lexer source-text fixtures.

## Required Inputs

- Milestone: `docs/milestones/M0007-lexer-implementation.md`
- Spec sections:
  - `docs/SPEC.md`
- ADRs:
  - `docs/adr/`
- Existing files:
  - `docs/ambiguities/M0006-lexical-grammar.md`
  - `docs/lexer/token-model.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - `docs/tests/m0007-lexical-grammar-proposal.sh` verifies the draft proposal exists and contains required ADR sections.
- Negative tests:
  - The validation script must fail before implementation because the proposal is absent.
- Diagnostic tests:
  - The proposal must discuss diagnostic consequences without defining diagnostic implementation.
- Adversarial tests:
  - Confirm the proposal remains under `docs/adr/proposals/` and is marked non-authoritative.
  - Confirm no lexer implementation is introduced.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0007-lexical-grammar-proposal.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Draft lexical grammar ADR proposal does not exist yet.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add `docs/adr/proposals/ADR-0021-lexical-grammar.md` as a draft proposal only. Do not update `docs/SPEC.md`, accepted ADRs, or ambiguity resolution state.

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

- Generate tests: `create docs/tests/m0007-lexical-grammar-proposal.sh`
- Verify tests fail: `docs/tests/m0007-lexical-grammar-proposal.sh`
- Ordinary tests: `docs/tests/m0007-lexical-grammar-proposal.sh && docs/tests/m0007-lexer-blocked.sh && docs/tests/m0006-token-model-fixtures.sh`
- Adversarial tests: `docs/tests/m0007-lexical-grammar-proposal.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0007-002-lexical-grammar-proposal.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0007-lexical-grammar-proposal.sh && docs/tests/m0007-lexer-blocked.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0007-lexical-grammar-proposal.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/adr/proposals/ADR-0021-lexical-grammar.md`
  - `docs/tasks/M0007-002-lexical-grammar-proposal.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted files under `docs/adr/`.
- Do not close `docs/ambiguities/M0006-lexical-grammar.md`.
- Do not implement lexer code.
- Do not add concrete lexer fixtures.

## Ambiguities And Dependencies

- Blocking ambiguity remains `docs/ambiguities/M0006-lexical-grammar.md`.
- Chief Architect approval is required before the proposal can become accepted source of truth.

## Execution Log

Append entries as the task progresses.

```text
2026-07-09 agent=Task-Decomposer phase=create-task result=pass notes=Created M0007 lexical grammar ADR proposal task.
2026-07-09 agent=Test-Engineer phase=generate-tests result=pass notes=Created docs/tests/m0007-lexical-grammar-proposal.sh before adding proposal.
2026-07-09 agent=Test-Engineer phase=verify-tests-fail result=pass notes=Validation failed as expected: missing docs/adr/proposals/ADR-0021-lexical-grammar.md.
2026-07-09 agent=Language-Designer phase=ordinary-tests result=pass notes=Proposal validation and M0007 blocked-state validation passed.
2026-07-09 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=Validation confirms proposal remains non-authoritative and no lexer code was added.
2026-07-09 agent=Reviewer phase=review result=pass notes=Self-review approved as draft-only unblock package; M0007 remains blocked pending Chief Architect approval.
2026-07-09 agent=Language-Designer phase=ci result=pass notes=Full CI-equivalent gate passed.
```

## Handoff

- Next Agent: `Language Designer`
- Reason: `Draft a non-authoritative lexical grammar ADR proposal for review.`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - `docs/ambiguities/M0006-lexical-grammar.md`
  - `docs/lexer/token-model.md`
