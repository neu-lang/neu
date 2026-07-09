# Task: M0011-003 Review Draft Declaration Syntax Proposal

## Task Metadata

- Task ID: `M0011-003`
- Milestone: `M0011`
- Milestone File: `docs/milestones/M0011-declaration-parser.md`
- Status: `complete`
- Owner Agent: `Reviewer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-09`
- Branch: `task/M0011-003-declaration-syntax-proposal-review`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/`
- Project Rules: `docs/AGENTS.md`
- Agent Prompts:
  - `.codex/agents/reviewer.md`
  - `.codex/agents/language-lawyer.md`
  - `.codex/agents/diagnostics-engineer.md`
  - `.codex/agents/simplicity-guardian.md`
  - `.codex/agents/chief-architect.md`

## Goal

Review draft ADR-0022 declaration syntax proposal and record required findings before any source-of-truth acceptance.

## Motivation

M0011 remains blocked until declaration syntax is accepted. ADR-0022 is currently a draft proposal and needs specialty review before Chief Architect can approve, reject, or request revisions.

## Scope

- Add ADR-0022 proposal review artifacts.
- Add a pending Chief Architect decision artifact.
- Validate that ADR-0022 remains non-authoritative.
- Keep declaration syntax ambiguity open.
- Keep parser implementation blocked.

## Out Of Scope

- Accepting ADR-0022.
- Modifying `docs/SPEC.md`.
- Moving ADR-0022 into accepted `docs/adr/`.
- Closing `docs/ambiguities/M0008-declaration-syntax.md`.
- Implementing parser code.
- Adding parser fixtures.
- Adding concrete declaration AST nodes.

## Required Inputs

- Milestone: `docs/milestones/M0011-declaration-parser.md`
- Spec sections:
  - `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0012-pattern-matching-and-algebraic-data.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
  - `docs/adr/ADR-0021-lexical-grammar.md`
- Existing files:
  - `docs/adr/proposals/ADR-0022-declaration-syntax.md`
  - `docs/ambiguities/M0008-declaration-syntax.md`
  - `docs/syntax/grammar-authority-ledger.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - `docs/tests/m0011-declaration-syntax-review.sh` verifies required review artifacts exist.
- Negative tests:
  - The validation script must fail before review artifacts are added.
- Diagnostic tests:
  - Diagnostics review must mention declaration diagnostics and ADR-0015.
- Adversarial tests:
  - Confirm Chief Architect decision remains pending.
  - Confirm ambiguity remains open.
  - Confirm no parser code, parser fixtures, or declaration AST nodes are introduced.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0011-declaration-syntax-review.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - ADR-0022 review artifacts do not exist yet.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add review files under `docs/adr/proposals/reviews/` for ADR-0022. The Chief Architect decision must remain pending.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing review package.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] Reviewer compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [x] M0011 milestone checklist is not marked complete because parser implementation remains blocked.

## Execution Commands

- Generate tests: `create docs/tests/m0011-declaration-syntax-review.sh`
- Verify tests fail: `docs/tests/m0011-declaration-syntax-review.sh`
- Ordinary tests: `docs/tests/m0011-declaration-syntax-review.sh && docs/tests/m0011-declaration-syntax-proposal.sh && docs/tests/m0011-declaration-parser-blocked.sh`
- Adversarial tests: `docs/tests/m0011-declaration-syntax-review.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0011-003-declaration-syntax-proposal-review.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0011-declaration-syntax-review.sh && docs/tests/m0011-declaration-syntax-proposal.sh && docs/tests/m0011-declaration-parser-blocked.sh && docs/tests/m0010-parser-recovery-architecture.sh && docs/tests/m0009-ast-data-model.sh && docs/tests/m0008-grammar-authority-ledger.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0007-blocker-status-sync.sh && docs/tests/m0007-language-designer-review.sh && docs/tests/m0007-lexical-grammar-review.sh && docs/tests/m0007-lexical-grammar-proposal.sh && docs/tests/m0007-lexer-blocked.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0011-declaration-syntax-review.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/tasks/M0011-003-declaration-syntax-proposal-review.md`
  - `docs/adr/proposals/reviews/ADR-0022-language-lawyer-review.md`
  - `docs/adr/proposals/reviews/ADR-0022-diagnostics-review.md`
  - `docs/adr/proposals/reviews/ADR-0022-simplicity-review.md`
  - `docs/adr/proposals/reviews/ADR-0022-chief-architect-decision.md`

## Forbidden Changes

- Do not accept ADR-0022.
- Do not modify `docs/SPEC.md`.
- Do not modify accepted ADR files.
- Do not close declaration syntax ambiguity.
- Do not implement parser code.
- Do not add parser fixtures.
- Do not add concrete declaration AST nodes.

## Ambiguities And Dependencies

- Blocking ambiguity remains `docs/ambiguities/M0008-declaration-syntax.md`.
- Chief Architect approval is required before ADR-0022 can become accepted source of truth.

## Execution Log

```text
2026-07-09 agent=Task-Decomposer phase=create-task result=pass notes=Created ADR-0022 review task.
2026-07-09 agent=Test-Engineer phase=generate-tests result=pass notes=Created docs/tests/m0011-declaration-syntax-review.sh before adding review artifacts.
2026-07-09 agent=Test-Engineer phase=verify-tests-fail result=pass notes=Validation failed as expected: missing ADR-0022 review artifacts.
2026-07-09 agent=Reviewer phase=ordinary-tests result=pass notes=ADR-0022 review validation passed after adding required review artifacts.
2026-07-09 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=Validation confirms reviews do not accept syntax and no parser code or fixtures were added.
2026-07-09 agent=Reviewer phase=review result=pass notes=Review approved ADR-0022 review package as scoped blocker-resolution progress.
2026-07-09 agent=Build-Engineer phase=ci result=pass notes=Full CI-equivalent gate passed.
```

## Handoff

- Next Agent: `Reviewer`
- Reason: `Review draft ADR-0022 without accepting declaration syntax.`
- Required Context:
  - This task file
  - `docs/adr/proposals/ADR-0022-declaration-syntax.md`
  - `docs/ambiguities/M0008-declaration-syntax.md`
  - `docs/milestones/M0011-declaration-parser.md`
