# Task: M0013-002 Draft expression statement and pattern syntax proposal

## Task Metadata

- Task ID: `M0013-002`
- Milestone: `M0013`
- Milestone File: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
- Status: `complete`
- Owner Agent: `Language Designer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0013-002-expression-statement-pattern-syntax-proposal`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0007-error-handling.md`
  - `docs/adr/ADR-0008-structured-concurrency-semantics.md`
  - `docs/adr/ADR-0009-async-suspension-and-borrowing.md`
  - `docs/adr/ADR-0011-flow-typing-and-smart-casts.md`
  - `docs/adr/ADR-0012-pattern-matching-and-algebraic-data.md`
  - `docs/adr/ADR-0018-unsafe-ffi-and-trust-boundaries.md`
  - `docs/adr/ADR-0021-lexical-grammar.md`
- Project Rules: `docs/AGENTS.md`
- Agent Prompts:
  - `.codex/agents/language-designer.md`
  - `.codex/agents/language-lawyer.md`
  - `.codex/agents/reviewer.md`

## Goal

Draft a non-authoritative ADR proposal for expression, statement, and pattern syntax so M0013 can move toward accepted grammar without parser implementation.

## Motivation

M0013 is blocked because semantic ADRs do not define concrete grammar for expressions, statements, blocks, patterns, coroutine syntax, unsafe syntax, precedence, associativity, recovery, or diagnostics. A proposal is the next safe step before review and acceptance.

## Scope

- Add `docs/adr/proposals/ADR-0024-expression-statement-pattern-syntax.md`.
- Include competing designs, trade-offs, recommended draft direction, required accepted content, required diagnostics, explicit deferrals, downstream consequences, and dependencies.
- Preserve `docs/ambiguities/M0008-expression-statement-pattern-syntax.md` as open.
- Keep `docs/SPEC.md` and accepted ADRs unchanged.

## Out Of Scope

- Accepted grammar.
- Parser fixtures.
- Parser implementation.
- Expression, statement, or pattern AST nodes.
- Resolving the ambiguity.
- Changing `docs/SPEC.md` or accepted ADRs.
- Defining type checking, ownership, borrowing, exhaustiveness, coroutine, or unsafe semantics.

## Required Inputs

- Milestone: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
- Ambiguity report: `docs/ambiguities/M0008-expression-statement-pattern-syntax.md`
- Ledger: `docs/syntax/grammar-authority-ledger.md`
- Existing ADRs listed under Source Of Truth.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Proposal file exists and is explicitly non-authoritative.
- Negative tests:
  - No accepted ADR, spec section, parser fixtures, AST nodes, or parser implementation are added.
- Diagnostic tests:
  - Proposal lists diagnostic obligations required before acceptance.
- Adversarial tests:
  - Proposal explicitly prevents parser implementation from depending on it.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0013-expression-statement-pattern-syntax-proposal.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Proposal file does not exist yet.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Create a planning proposal only. Do not move it into accepted ADRs, update `docs/SPEC.md`, add fixtures, or implement parser behavior.

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

- Generate tests: `create docs/tests/m0013-expression-statement-pattern-syntax-proposal.sh`
- Verify tests fail: `docs/tests/m0013-expression-statement-pattern-syntax-proposal.sh`
- Ordinary tests: `docs/tests/m0013-expression-statement-pattern-syntax-proposal.sh && docs/tests/m0013-expression-statement-pattern-parser-blocked.sh`
- Adversarial tests: `docs/tests/m0013-expression-statement-pattern-syntax-proposal.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0013-002-expression-statement-pattern-syntax-proposal.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0013-expression-statement-pattern-syntax-proposal.sh && docs/tests/m0013-expression-statement-pattern-parser-blocked.sh && docs/tests/m0012-type-generic-parser-implementation.sh && docs/tests/m0012-type-ast-shell.sh && docs/tests/m0012-type-generic-parser-fixtures.sh && docs/tests/m0012-type-generic-syntax-accepted.sh && docs/tests/m0012-type-generic-syntax-concrete-draft.sh && docs/tests/m0012-type-generic-syntax-review.sh && docs/tests/m0012-type-generic-syntax-proposal.sh && docs/tests/m0012-type-generic-parser-blocked.sh && docs/tests/m0011-declaration-parser-implementation.sh && docs/tests/m0011-declaration-ast-shell.sh && docs/tests/m0011-declaration-parser-fixtures.sh && docs/tests/m0011-declaration-syntax-accepted.sh && docs/tests/m0010-parser-recovery-architecture.sh && docs/tests/m0009-ast-data-model.sh && docs/tests/m0008-grammar-authority-ledger.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0013-expression-statement-pattern-syntax-proposal.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/adr/proposals/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/tasks/M0013-002-expression-statement-pattern-syntax-proposal.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted ADRs under `docs/adr/`.
- Do not weaken or delete failing tests without reviewer approval.
- Do not implement work outside this task scope.
- Do not introduce accepted language semantics.
- Do not add expression, statement, pattern, coroutine, or unsafe parser APIs.
- Do not add expression, statement, or pattern AST nodes or fixtures.

## Ambiguities And Dependencies

- M0013 remains blocked until Chief Architect accepts a future ADR or `docs/SPEC.md` revision.
- This proposal must be reviewed by Language Lawyer, Adversarial Engineer, Diagnostics Engineer, and Simplicity Guardian before acceptance.

## Execution Log

Append entries as the task progresses.

```text
2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created M0013 expression/statement/pattern syntax proposal task.
2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Created docs/tests/m0013-expression-statement-pattern-syntax-proposal.sh before drafting proposal.
2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=Proposal validator failed before implementation because ADR-0024 proposal did not exist.
2026-07-10 agent=Language-Designer phase=implementation result=pass notes=Drafted non-authoritative ADR-0024 proposal without changing accepted source of truth.
2026-07-10 agent=Test-Engineer phase=ordinary-tests result=pass notes=M0013 proposal and blocker validators passed.
2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0013-002-soundness.md after ordinary-tests evidence.
2026-07-10 agent=Reviewer phase=review result=pass notes=docs/tasks/reviews/M0013-002-review.md approves proposal-only scope.
2026-07-10 agent=Build-Engineer phase=ci result=pass notes=Full M0013-M0002 validation command passed.
```

## Handoff

- Next Agent: `Language Lawyer`
- Reason: `Review the non-authoritative ADR-0024 proposal before concrete grammar drafting or acceptance.`
- Required Context:
  - This task file
  - `docs/adr/proposals/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/ambiguities/M0008-expression-statement-pattern-syntax.md`
  - `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
