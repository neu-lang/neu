# Task: M0007-001 Record Lexer Implementation Lexical Grammar Blocker

## Task Metadata

- Task ID: `M0007-001`
- Milestone: `M0007`
- Milestone File: `docs/milestones/M0007-lexer-implementation.md`
- Status: `blocked`
- Owner Agent: `Language Lawyer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-09`
- Branch: `task/M0007-001-lexical-grammar-blocker`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/`
- Project Rules: `docs/AGENTS.md`
- Agent Prompts:
  - `.codex/agents/task-decomposer.md`
  - `.codex/agents/language-lawyer.md`
  - `.codex/agents/language-designer.md`
  - `.codex/agents/reviewer.md`

## Goal

Record that M0007 lexer implementation is blocked until detailed lexical grammar is accepted.

## Motivation

M0007 requires lexer fixtures from M0006 and spec-backed tokenization rules. M0006 explicitly recorded that detailed lexical grammar is missing.

## Scope

- Record the M0007 blocker.
- Add a validation script proving the ambiguity report is open.
- Confirm no lexer implementation files exist.
- Define the handoff to Language Designer and Chief Architect.

## Out Of Scope

- Lexer implementation.
- Token enum or token model code.
- Concrete lexer fixtures with source text.
- Lexical diagnostics.
- Syntax or lexical grammar decisions.

## Required Inputs

- Milestone: `docs/milestones/M0007-lexer-implementation.md`
- Spec sections:
  - `docs/SPEC.md`
- ADRs:
  - `docs/adr/`
- Existing files:
  - `docs/ambiguities/M0006-lexical-grammar.md`
  - `docs/lexer/token-model.md`
  - `tests/fixtures/lexer/M0006-inert.fixture.toml`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - `docs/tests/m0007-lexer-blocked.sh` verifies the M0006 lexical grammar ambiguity is open and blocks M0007.
- Negative tests:
  - The validation script fails if lexer implementation files exist before grammar authority is accepted.
- Diagnostic tests:
  - Not applicable; M0007 diagnostics remain blocked.
- Adversarial tests:
  - Confirm no lexer code, token code, or concrete lexer fixtures were added.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0007-lexer-blocked.sh`
- Expected pre-implementation result: `pass`
- Failure reason expected before implementation:
  - Not applicable. This is a blocker-record task, not implementation.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Do not implement lexer code. Add only the validation script and blocker record, then hand off to Language Designer for lexical grammar ADR/spec work.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason or are marked not applicable for blocker-record work.
- [x] Implementation is the smallest passing change.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] Reviewer compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [ ] Milestone checklist is updated.

## Execution Commands

- Generate tests: `create docs/tests/m0007-lexer-blocked.sh`
- Verify tests fail: `not applicable: blocker-record task`
- Ordinary tests: `docs/tests/m0007-lexer-blocked.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`
- Adversarial tests: `docs/tests/m0007-lexer-blocked.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0007-001-lexical-grammar-blocker.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0007-lexer-blocked.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0007-lexer-blocked.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/tasks/M0007-001-lexical-grammar-blocker.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not implement lexer code.
- Do not add `crates/newlang/src/lexer.rs`.
- Do not add `crates/newlang/src/token.rs`.
- Do not add concrete lexer fixtures.

## Ambiguities And Dependencies

- Blocking ambiguity: `docs/ambiguities/M0006-lexical-grammar.md`.
- Required next owner: `Language Designer`.
- Required final resolver: `Chief Architect`.

## Execution Log

Append entries as the task progresses.

```text
2026-07-09 agent=Task-Decomposer phase=create-task result=pass notes=Created M0007 blocker task because lexical grammar is unresolved.
2026-07-09 agent=Language-Lawyer phase=ordinary-tests result=pass notes=docs/tests/m0007-lexer-blocked.sh and prior milestone gates passed.
2026-07-09 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=Blocked-state validation confirms no lexer implementation or concrete lexer fixtures exist.
```

## Handoff

- Next Agent: `Language Designer`
- Reason: `M0007 implementation is blocked until lexical grammar is specified through ADR/spec workflow.`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - `docs/ambiguities/M0006-lexical-grammar.md`
  - `docs/milestones/M0007-lexer-implementation.md`
