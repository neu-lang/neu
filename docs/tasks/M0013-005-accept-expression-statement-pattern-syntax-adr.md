# Task: M0013-005 Accept ADR-0024 expression statement and pattern syntax

## Task Metadata

- Task ID: `M0013-005`
- Milestone: `M0013`
- Milestone File: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
- Status: `complete`
- Owner Agent: `Chief Architect`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0013-005-accept-expression-statement-pattern-syntax-adr`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/proposals/ADR-0024-expression-statement-pattern-syntax.md`
- Project Rules: `docs/AGENTS.md`
- Agent Prompts:
  - `.codex/agents/chief-architect.md`
  - `.codex/agents/language-lawyer.md`
  - `.codex/agents/spec-compliance-auditor.md`
  - `.codex/agents/reviewer.md`

## Goal

Accept ADR-0024 as expression, statement, and pattern syntax source of truth, resolve the M0013 syntax ambiguity, and unblock body parser fixture tasks without implementing parser behavior.

## Motivation

ADR-0024 now contains concrete draft grammar, precedence, diagnostics, recovery boundaries, attack cases, explicit deferrals, and review artifacts. M0013 parser fixtures and implementation require accepted grammar authority before proceeding.

## Scope

- Add accepted `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`.
- Add an ADR-0024 summary to `docs/SPEC.md`.
- Resolve `docs/ambiguities/M0008-expression-statement-pattern-syntax.md`.
- Update `docs/syntax/grammar-authority-ledger.md` for ADR-0024 body constructs.
- Update the Chief Architect decision artifact to approved.
- Update M0013 validators to distinguish historical blocker/proposal evidence from accepted authority.

## Out Of Scope

- Parser fixtures.
- Parser implementation.
- Expression, statement, or pattern AST nodes.
- Type checking, flow typing, exhaustiveness checking, ownership analysis, coroutine analysis, or unsafe checking.
- Unsafe block syntax and coroutine syntax, which remain deferred by ADR-0024.

## Required Inputs

- Milestone: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
- ADR proposal: `docs/adr/proposals/ADR-0024-expression-statement-pattern-syntax.md`
- Reviews: `docs/adr/proposals/reviews/ADR-0024-*.md`
- Ambiguity: `docs/ambiguities/M0008-expression-statement-pattern-syntax.md`
- Ledger: `docs/syntax/grammar-authority-ledger.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Accepted ADR-0024 exists, is marked accepted, and contains concrete grammar, precedence, diagnostics, and recovery boundaries.
- Negative tests:
  - Parser fixtures, AST nodes, and parser implementation remain absent during acceptance.
- Diagnostic tests:
  - ADR-0024 defines body syntax diagnostics with primary span and recovery action.
- Adversarial tests:
  - Unsafe block syntax and coroutine syntax remain deferred.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0013-expression-statement-pattern-syntax-accepted.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Accepted `docs/adr/ADR-0024-expression-statement-pattern-syntax.md` does not exist yet.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Promote the reviewed concrete draft into an accepted ADR and update only source-of-truth and governance documents. Leave parser fixtures, AST nodes, and parser implementation for later M0013 tasks.

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

- Generate tests: `create docs/tests/m0013-expression-statement-pattern-syntax-accepted.sh`
- Verify tests fail: `docs/tests/m0013-expression-statement-pattern-syntax-accepted.sh`
- Ordinary tests: `docs/tests/m0013-expression-statement-pattern-syntax-accepted.sh && docs/tests/m0013-expression-statement-pattern-syntax-concrete-draft.sh && docs/tests/m0013-expression-statement-pattern-syntax-review.sh && docs/tests/m0013-expression-statement-pattern-syntax-proposal.sh && docs/tests/m0013-expression-statement-pattern-parser-blocked.sh`
- Adversarial tests: `docs/tests/m0013-expression-statement-pattern-syntax-accepted.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0013-005-accept-expression-statement-pattern-syntax-adr.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0013-expression-statement-pattern-syntax-accepted.sh && docs/tests/m0013-expression-statement-pattern-syntax-concrete-draft.sh && docs/tests/m0013-expression-statement-pattern-syntax-review.sh && docs/tests/m0013-expression-statement-pattern-syntax-proposal.sh && docs/tests/m0013-expression-statement-pattern-parser-blocked.sh && docs/tests/m0012-type-generic-parser-implementation.sh && docs/tests/m0012-type-ast-shell.sh && docs/tests/m0012-type-generic-parser-fixtures.sh && docs/tests/m0012-type-generic-syntax-accepted.sh && docs/tests/m0012-type-generic-syntax-concrete-draft.sh && docs/tests/m0012-type-generic-syntax-review.sh && docs/tests/m0012-type-generic-syntax-proposal.sh && docs/tests/m0012-type-generic-parser-blocked.sh && docs/tests/m0011-declaration-parser-implementation.sh && docs/tests/m0011-declaration-ast-shell.sh && docs/tests/m0011-declaration-parser-fixtures.sh && docs/tests/m0011-declaration-syntax-accepted.sh && docs/tests/m0010-parser-recovery-architecture.sh && docs/tests/m0009-ast-data-model.sh && docs/tests/m0008-grammar-authority-ledger.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0013-expression-statement-pattern-syntax-accepted.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/SPEC.md`
  - `docs/ambiguities/M0008-expression-statement-pattern-syntax.md`
  - `docs/syntax/grammar-authority-ledger.md`
  - `docs/adr/proposals/reviews/ADR-0024-chief-architect-decision.md`
  - M0013 validators and task metadata

## Forbidden Changes

- Do not add parser fixtures.
- Do not add parser implementation.
- Do not add expression, statement, or pattern AST nodes.
- Do not implement unsafe block syntax or coroutine syntax.
- Do not implement type checking, flow typing, exhaustiveness, ownership, borrowing, coroutine, or unsafe analysis.
- Do not introduce language semantics outside ADR-0024.

## Ambiguities And Dependencies

- Unsafe block syntax and coroutine syntax remain deferred.
- Match or `when` syntax remains deferred.

## Execution Log

Append entries as the task progresses.

```text
2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created ADR-0024 acceptance task.
2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Created accepted-state validator before accepting ADR-0024.
2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=Accepted-state validator failed before implementation because docs/adr/ADR-0024-expression-statement-pattern-syntax.md did not exist.
2026-07-10 agent=Chief-Architect phase=implementation result=pass notes=Promoted reviewed concrete draft to accepted ADR-0024, updated SPEC, resolved ambiguity, updated ledger, and approved decision artifact without parser implementation.
2026-07-10 agent=Test-Engineer phase=ordinary-tests result=pass notes=M0013 accepted-state and historical validators passed.
2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0013-005-soundness.md after ordinary-tests evidence.
2026-07-10 agent=Reviewer phase=review result=pass notes=docs/tasks/reviews/M0013-005-review.md approves source-of-truth acceptance scope.
2026-07-10 agent=Build-Engineer phase=ci result=pass notes=Full M0013-M0002 validation command passed.
```

## Handoff

- Next Agent: `Test Engineer`
- Reason: `Create concrete body parser fixtures after accepted source-of-truth update.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
