# Task: M0011-002 Draft Declaration Syntax ADR Proposal

## Task Metadata

- Task ID: `M0011-002`
- Milestone: `M0011`
- Milestone File: `docs/milestones/M0011-declaration-parser.md`
- Status: `complete`
- Owner Agent: `Language Designer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-09`
- Branch: `task/M0011-002-declaration-syntax-proposal`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/`
- Project Rules: `docs/AGENTS.md`
- Agent Prompts:
  - `.codex/agents/language-designer.md`
  - `.codex/agents/language-lawyer.md`
  - `.codex/agents/reviewer.md`

## Goal

Draft a non-authoritative declaration syntax ADR proposal that can be reviewed before M0011 declaration parser implementation.

## Motivation

M0011 is blocked by `docs/ambiguities/M0008-declaration-syntax.md`. A proposal is needed before the project can accept declaration parser fixtures or implementation.

## Scope

- Add a draft declaration syntax ADR proposal under `docs/adr/proposals/`.
- Include question, competing designs, trade-offs, recommended draft, required accepted content, downstream consequences, and dependencies.
- Keep the declaration syntax ambiguity open.
- Keep M0011 parser implementation blocked.

## Out Of Scope

- Modifying `docs/SPEC.md`.
- Moving the proposal into accepted `docs/adr/`.
- Resolving `docs/ambiguities/M0008-declaration-syntax.md`.
- Implementing parser code.
- Adding concrete parser fixtures.
- Adding concrete declaration AST nodes.

## Required Inputs

- Milestone: `docs/milestones/M0011-declaration-parser.md`
- Spec sections:
  - `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0012-pattern-matching-and-algebraic-data.md`
  - `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
  - `docs/adr/ADR-0021-lexical-grammar.md`
- Existing files:
  - `docs/ambiguities/M0008-declaration-syntax.md`
  - `docs/syntax/grammar-authority-ledger.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - `docs/tests/m0011-declaration-syntax-proposal.sh` verifies the draft proposal exists and is non-authoritative.
- Negative tests:
  - The validation script must fail before the proposal exists.
- Diagnostic tests:
  - Proposal must identify declaration diagnostic consequences without defining implementation details.
- Adversarial tests:
  - Confirm the ambiguity remains open.
  - Confirm no parser code or parser fixtures are introduced.
  - Confirm the proposal is not accepted source of truth.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0011-declaration-syntax-proposal.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Draft declaration syntax ADR proposal does not exist yet.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add `docs/adr/proposals/ADR-0022-declaration-syntax.md` as a draft planning artifact only. Do not update source-of-truth files.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing proposal artifact.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] Reviewer compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [x] M0011 milestone checklist is not marked complete because parser implementation remains blocked.

## Execution Commands

- Generate tests: `create docs/tests/m0011-declaration-syntax-proposal.sh`
- Verify tests fail: `docs/tests/m0011-declaration-syntax-proposal.sh`
- Ordinary tests: `docs/tests/m0011-declaration-syntax-proposal.sh && docs/tests/m0011-declaration-parser-blocked.sh`
- Adversarial tests: `docs/tests/m0011-declaration-syntax-proposal.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0011-002-declaration-syntax-proposal.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0011-declaration-syntax-proposal.sh && docs/tests/m0011-declaration-parser-blocked.sh && docs/tests/m0010-parser-recovery-architecture.sh && docs/tests/m0009-ast-data-model.sh && docs/tests/m0008-grammar-authority-ledger.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0007-blocker-status-sync.sh && docs/tests/m0007-language-designer-review.sh && docs/tests/m0007-lexical-grammar-review.sh && docs/tests/m0007-lexical-grammar-proposal.sh && docs/tests/m0007-lexer-blocked.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0011-declaration-syntax-proposal.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/tasks/M0011-002-declaration-syntax-proposal.md`
  - `docs/adr/proposals/ADR-0022-declaration-syntax.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted files under `docs/adr/`.
- Do not close `docs/ambiguities/M0008-declaration-syntax.md`.
- Do not implement parser code.
- Do not add parser fixtures.
- Do not add concrete declaration AST nodes.

## Ambiguities And Dependencies

- Blocking ambiguity remains `docs/ambiguities/M0008-declaration-syntax.md`.
- Chief Architect approval is required before the proposal can become accepted source of truth.

## Execution Log

```text
2026-07-09 agent=Task-Decomposer phase=create-task result=pass notes=Created M0011 declaration syntax proposal task.
2026-07-09 agent=Test-Engineer phase=generate-tests result=pass notes=Created docs/tests/m0011-declaration-syntax-proposal.sh before adding proposal.
2026-07-09 agent=Test-Engineer phase=verify-tests-fail result=pass notes=Validation failed as expected: missing docs/adr/proposals/ADR-0022-declaration-syntax.md.
2026-07-09 agent=Language-Designer phase=ordinary-tests result=pass notes=M0011 declaration syntax proposal validation passed after adding draft proposal.
2026-07-09 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=Validation confirms proposal remains non-authoritative and no parser code or fixtures were added.
2026-07-09 agent=Reviewer phase=review result=pass notes=Review approved draft proposal as scoped blocker-resolution progress.
2026-07-09 agent=Build-Engineer phase=ci result=pass notes=Full CI-equivalent gate passed.
```

## Handoff

- Next Agent: `Language Designer`
- Reason: `Draft non-authoritative declaration syntax proposal for review.`
- Required Context:
  - This task file
  - `docs/ambiguities/M0008-declaration-syntax.md`
  - `docs/syntax/grammar-authority-ledger.md`
  - `docs/milestones/M0011-declaration-parser.md`
