# Task: M0013-003 Review expression statement and pattern syntax proposal

## Task Metadata

- Task ID: `M0013-003`
- Milestone: `M0013`
- Milestone File: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
- Status: `complete`
- Owner Agent: `Language Lawyer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0013-003-expression-statement-pattern-syntax-proposal-review`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/proposals/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/ADR-0007-error-handling.md`
  - `docs/adr/ADR-0008-structured-concurrency-semantics.md`
  - `docs/adr/ADR-0009-async-suspension-and-borrowing.md`
  - `docs/adr/ADR-0011-flow-typing-and-smart-casts.md`
  - `docs/adr/ADR-0012-pattern-matching-and-algebraic-data.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0018-unsafe-ffi-and-trust-boundaries.md`
  - `docs/adr/ADR-0021-lexical-grammar.md`
- Project Rules: `docs/AGENTS.md`
- Agent Prompts:
  - `.codex/agents/language-lawyer.md`
  - `.codex/agents/adversarial-engineer.md`
  - `.codex/agents/diagnostics-engineer.md`
  - `.codex/agents/simplicity-guardian.md`
  - `.codex/agents/chief-architect.md`

## Goal

Review ADR-0024 as a non-authoritative expression, statement, and pattern syntax proposal and identify what must be revised before acceptance.

## Motivation

ADR-0024 currently defines a direction and acceptance checklist, not concrete grammar. Review must prevent parser implementation from depending on it and must clarify which missing decisions block acceptance.

## Scope

- Add Language Lawyer review for semantic consistency and missing grammar authority.
- Add Adversarial Engineer review for ownership scope, coroutine, unsafe, pattern, and soundness risks.
- Add Diagnostics Engineer review for diagnostic and recovery obligations.
- Add Simplicity Guardian review for scope and abstraction control.
- Add Chief Architect decision artifact preserving pending status.

## Out Of Scope

- Revising ADR-0024 concrete grammar.
- Accepting ADR-0024.
- Changing `docs/SPEC.md`.
- Creating parser fixtures.
- Adding AST nodes or parser implementation.
- Resolving `docs/ambiguities/M0008-expression-statement-pattern-syntax.md`.

## Required Inputs

- Milestone: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
- Proposal: `docs/adr/proposals/ADR-0024-expression-statement-pattern-syntax.md`
- Ambiguity report: `docs/ambiguities/M0008-expression-statement-pattern-syntax.md`
- Ledger: `docs/syntax/grammar-authority-ledger.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - All required review artifacts exist and request revision before acceptance.
- Negative tests:
  - No accepted ADR, spec section, parser fixtures, AST nodes, or parser implementation are added.
- Diagnostic tests:
  - Diagnostics review requests concrete diagnostic categories, primary spans, recovery actions, and safe suggestions.
- Adversarial tests:
  - Reviews keep M0013 blocked and reject semantic inference from proposal text.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0013-expression-statement-pattern-syntax-review.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - ADR-0024 review artifacts do not exist yet.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Create review artifacts only. Keep ADR-0024 as a draft proposal and leave the ambiguity open.

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
- [x] Milestone checklist is updated.

## Execution Commands

- Generate tests: `create docs/tests/m0013-expression-statement-pattern-syntax-review.sh`
- Verify tests fail: `docs/tests/m0013-expression-statement-pattern-syntax-review.sh`
- Ordinary tests: `docs/tests/m0013-expression-statement-pattern-syntax-review.sh && docs/tests/m0013-expression-statement-pattern-syntax-proposal.sh && docs/tests/m0013-expression-statement-pattern-parser-blocked.sh`
- Adversarial tests: `docs/tests/m0013-expression-statement-pattern-syntax-review.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0013-003-expression-statement-pattern-syntax-proposal-review.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0013-expression-statement-pattern-syntax-review.sh && docs/tests/m0013-expression-statement-pattern-syntax-proposal.sh && docs/tests/m0013-expression-statement-pattern-parser-blocked.sh && docs/tests/m0012-type-generic-parser-implementation.sh && docs/tests/m0012-type-ast-shell.sh && docs/tests/m0012-type-generic-parser-fixtures.sh && docs/tests/m0012-type-generic-syntax-accepted.sh && docs/tests/m0012-type-generic-syntax-concrete-draft.sh && docs/tests/m0012-type-generic-syntax-review.sh && docs/tests/m0012-type-generic-syntax-proposal.sh && docs/tests/m0012-type-generic-parser-blocked.sh && docs/tests/m0011-declaration-parser-implementation.sh && docs/tests/m0011-declaration-ast-shell.sh && docs/tests/m0011-declaration-parser-fixtures.sh && docs/tests/m0011-declaration-syntax-accepted.sh && docs/tests/m0010-parser-recovery-architecture.sh && docs/tests/m0009-ast-data-model.sh && docs/tests/m0008-grammar-authority-ledger.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0013-expression-statement-pattern-syntax-review.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/adr/proposals/reviews/ADR-0024-language-lawyer-review.md`
  - `docs/adr/proposals/reviews/ADR-0024-adversarial-review.md`
  - `docs/adr/proposals/reviews/ADR-0024-diagnostics-review.md`
  - `docs/adr/proposals/reviews/ADR-0024-simplicity-review.md`
  - `docs/adr/proposals/reviews/ADR-0024-chief-architect-decision.md`
  - `docs/tasks/M0013-003-expression-statement-pattern-syntax-proposal-review.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted ADRs under `docs/adr/`.
- Do not weaken or delete failing tests without reviewer approval.
- Do not implement work outside this task scope.
- Do not accept ADR-0024.
- Do not add expression, statement, pattern, coroutine, or unsafe parser APIs.
- Do not add expression, statement, or pattern AST nodes or fixtures.

## Ambiguities And Dependencies

- ADR-0024 lacks concrete grammar and remains a proposal.
- M0013 remains blocked until accepted source of truth exists.

## Execution Log

Append entries as the task progresses.

```text
2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created M0013 ADR-0024 review task.
2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Created docs/tests/m0013-expression-statement-pattern-syntax-review.sh before adding review artifacts.
2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=Review validator failed before implementation because ADR-0024 review artifacts did not exist.
2026-07-10 agent=Language-Lawyer phase=implementation result=pass notes=Added required review artifacts and kept ADR-0024 pending.
2026-07-10 agent=Test-Engineer phase=ordinary-tests result=pass notes=M0013 review, proposal, and blocker validators passed.
2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0013-003-soundness.md after ordinary-tests evidence.
2026-07-10 agent=Reviewer phase=review result=pass notes=docs/tasks/reviews/M0013-003-review.md approves review-only scope.
2026-07-10 agent=Build-Engineer phase=ci result=pass notes=Full M0013-M0002 validation command passed.
```

## Handoff

- Next Agent: `Language Designer`
- Reason: `Revise ADR-0024 into concrete draft grammar after reviews.`
- Required Context:
  - This task file
  - `docs/adr/proposals/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/proposals/reviews/ADR-0024-*.md`
