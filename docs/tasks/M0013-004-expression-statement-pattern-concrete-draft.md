# Task: M0013-004 Revise expression statement and pattern proposal with concrete draft grammar

## Task Metadata

- Task ID: `M0013-004`
- Milestone: `M0013`
- Milestone File: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
- Status: `complete`
- Owner Agent: `Language Designer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0013-004-expression-statement-pattern-concrete-draft`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/proposals/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/proposals/reviews/ADR-0024-*.md`
- Project Rules: `docs/AGENTS.md`
- Agent Prompts:
  - `.codex/agents/language-designer.md`
  - `.codex/agents/language-lawyer.md`
  - `.codex/agents/diagnostics-engineer.md`
  - `.codex/agents/adversarial-engineer.md`

## Goal

Revise draft ADR-0024 with concrete expression, statement, block, and pattern draft grammar plus diagnostics and recovery boundaries, while keeping it non-authoritative.

## Motivation

ADR-0024 reviews request concrete grammar before acceptance. This task moves the proposal from a direction-setting document to a reviewable concrete draft without accepting it or unblocking parser implementation.

## Scope

- Add concrete draft grammar sections to ADR-0024.
- Define a small bootstrap grammar for expressions, statements, blocks, and patterns.
- Include an operator precedence and associativity table.
- Include parser recovery boundaries.
- Include parser diagnostic categories with primary span, recovery action, and safe suggestion policy.
- Explicitly defer coroutine syntax and unsafe block syntax or define minimal forms.
- Keep `docs/ambiguities/M0008-expression-statement-pattern-syntax.md` open.

## Out Of Scope

- Accepting ADR-0024.
- Changing `docs/SPEC.md`.
- Creating parser fixtures.
- Adding AST nodes or parser implementation.
- Resolving the M0013 ambiguity.
- Defining type checking, ownership, borrow, flow typing, exhaustiveness, coroutine, or unsafe semantics.

## Required Inputs

- Proposal: `docs/adr/proposals/ADR-0024-expression-statement-pattern-syntax.md`
- Reviews: `docs/adr/proposals/reviews/ADR-0024-*.md`
- Ambiguity: `docs/ambiguities/M0008-expression-statement-pattern-syntax.md`
- Milestone: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Proposal contains concrete draft grammar, precedence, diagnostics, recovery, and attack cases.
- Negative tests:
  - Proposal remains non-authoritative and no accepted ADR, spec section, fixtures, AST nodes, or parser implementation are added.
- Diagnostic tests:
  - Diagnostic table includes primary span, recovery action, and safe suggestion policy.
- Adversarial tests:
  - Draft explicitly defers coroutine syntax and unsafe block syntax unless accepted later.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0013-expression-statement-pattern-syntax-concrete-draft.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - ADR-0024 does not yet contain concrete draft grammar.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Edit only the ADR-0024 proposal and task metadata. Do not create accepted source of truth, fixtures, AST nodes, or parser behavior.

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

- Generate tests: `create docs/tests/m0013-expression-statement-pattern-syntax-concrete-draft.sh`
- Verify tests fail: `docs/tests/m0013-expression-statement-pattern-syntax-concrete-draft.sh`
- Ordinary tests: `docs/tests/m0013-expression-statement-pattern-syntax-concrete-draft.sh && docs/tests/m0013-expression-statement-pattern-syntax-review.sh && docs/tests/m0013-expression-statement-pattern-syntax-proposal.sh && docs/tests/m0013-expression-statement-pattern-parser-blocked.sh`
- Adversarial tests: `docs/tests/m0013-expression-statement-pattern-syntax-concrete-draft.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0013-004-expression-statement-pattern-concrete-draft.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0013-expression-statement-pattern-syntax-concrete-draft.sh && docs/tests/m0013-expression-statement-pattern-syntax-review.sh && docs/tests/m0013-expression-statement-pattern-syntax-proposal.sh && docs/tests/m0013-expression-statement-pattern-parser-blocked.sh && docs/tests/m0012-type-generic-parser-implementation.sh && docs/tests/m0012-type-ast-shell.sh && docs/tests/m0012-type-generic-parser-fixtures.sh && docs/tests/m0012-type-generic-syntax-accepted.sh && docs/tests/m0012-type-generic-syntax-concrete-draft.sh && docs/tests/m0012-type-generic-syntax-review.sh && docs/tests/m0012-type-generic-syntax-proposal.sh && docs/tests/m0012-type-generic-parser-blocked.sh && docs/tests/m0011-declaration-parser-implementation.sh && docs/tests/m0011-declaration-ast-shell.sh && docs/tests/m0011-declaration-parser-fixtures.sh && docs/tests/m0011-declaration-syntax-accepted.sh && docs/tests/m0010-parser-recovery-architecture.sh && docs/tests/m0009-ast-data-model.sh && docs/tests/m0008-grammar-authority-ledger.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0013-expression-statement-pattern-syntax-concrete-draft.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/adr/proposals/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/tasks/M0013-004-expression-statement-pattern-concrete-draft.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted ADRs under `docs/adr/`.
- Do not weaken or delete failing tests without reviewer approval.
- Do not accept ADR-0024.
- Do not add parser fixtures, AST nodes, or parser implementation.
- Do not resolve the M0013 ambiguity.

## Ambiguities And Dependencies

- M0013 remains blocked until Chief Architect accepts ADR-0024 or a `docs/SPEC.md` revision.
- Concrete draft grammar will still need review before acceptance.

## Execution Log

Append entries as the task progresses.

```text
2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created M0013 concrete draft grammar task.
2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Created docs/tests/m0013-expression-statement-pattern-syntax-concrete-draft.sh before revising proposal.
2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=Concrete draft validator failed before implementation because ADR-0024 lacked concrete draft grammar.
2026-07-10 agent=Language-Designer phase=implementation result=pass notes=Revised ADR-0024 with concrete draft grammar, precedence, diagnostics, recovery, attack cases, and deferrals.
2026-07-10 agent=Test-Engineer phase=ordinary-tests result=pass notes=M0013 concrete draft, review, proposal, and blocker validators passed.
2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0013-004-soundness.md after ordinary-tests evidence.
2026-07-10 agent=Reviewer phase=review result=pass notes=docs/tasks/reviews/M0013-004-review.md approves concrete draft-only scope.
2026-07-10 agent=Build-Engineer phase=ci result=pass notes=Full M0013-M0002 validation command passed.
```

## Handoff

- Next Agent: `Chief Architect`
- Reason: `Decide whether concrete ADR-0024 draft is ready for acceptance or needs another review cycle.`
- Required Context:
  - This task file
  - `docs/adr/proposals/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/proposals/reviews/ADR-0024-*.md`
