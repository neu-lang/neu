# Task: M0012-005 Accept ADR-0023 type and generic syntax

## Task Metadata

- Task ID: `M0012-005`
- Milestone: `M0012`
- Milestone File: `docs/milestones/M0012-type-and-generic-syntax-parser.md`
- Status: `complete`
- Owner main task: `main task`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0012-005-accept-type-generic-syntax-adr`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
  - `docs/adr/proposals/ADR-0023-type-and-generic-syntax.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Accept ADR-0023 as type and generic syntax source of truth, resolve the M0012 type/generic ambiguity, and unblock type parser fixture and implementation tasks without implementing parser behavior.

## Motivation

M0012 is blocked until type, nullable, generic, function-type, and capability-bound syntax is accepted. The ADR-0023 draft now contains concrete grammar, diagnostics, recovery boundaries, attack-case decisions, and deferrals.

## Scope

- Add accepted `docs/adr/ADR-0023-type-and-generic-syntax.md`.
- Add an ADR-0023 summary to `docs/SPEC.md`.
- Resolve `docs/ambiguities/M0008-type-generic-syntax.md`.
- Update `docs/syntax/grammar-authority-ledger.md` for ADR-0023 type/generic constructs.
- Update the main task decision artifact to approved.
- Update M0012 validators to distinguish historical blocker/proposal evidence from accepted authority.

## Out Of Scope

- Type parser implementation.
- Type AST nodes.
- Type or generic parser fixtures.
- Type checking, generic constraint solving, or capability analysis.
- Expression, statement, pattern, coroutine, or unsafe syntax.

## Required Inputs

- Milestone: `docs/milestones/M0012-type-and-generic-syntax-parser.md`
- ADR proposal: `docs/adr/proposals/ADR-0023-type-and-generic-syntax.md`
- Reviews: `docs/adr/proposals/reviews/ADR-0023-*.md`
- Ambiguity: `docs/ambiguities/M0008-type-generic-syntax.md`
- Ledger: `docs/syntax/grammar-authority-ledger.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Accepted ADR-0023 exists, is marked accepted, and contains concrete grammar and diagnostics.
- Negative tests:
  - Type parser APIs, type AST nodes, and type/generic fixtures remain absent during acceptance.
- Diagnostic tests:
  - ADR-0023 defines type syntax diagnostics with primary span and recovery action.
- Adversarial tests:
  - Expression, statement, pattern, coroutine, and unsafe syntax remain unresolved.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0012-type-generic-syntax-accepted.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Accepted `docs/adr/ADR-0023-type-and-generic-syntax.md` does not exist yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Promote the reviewed concrete draft into an accepted ADR and update only source-of-truth and governance documents. Leave parser and AST implementation for later M0012 tasks.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing change.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] main-task review compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [x] Milestone checklist is updated.

## Execution Commands

- Generate tests: `create docs/tests/m0012-type-generic-syntax-accepted.sh`
- Verify tests fail: `docs/tests/m0012-type-generic-syntax-accepted.sh`
- Ordinary tests: `docs/tests/m0012-type-generic-syntax-accepted.sh && docs/tests/m0012-type-generic-syntax-concrete-draft.sh && docs/tests/m0012-type-generic-syntax-review.sh && docs/tests/m0012-type-generic-parser-blocked.sh`
- Adversarial tests: `docs/tests/m0012-type-generic-syntax-accepted.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0012-005-accept-type-generic-syntax-adr.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0012-type-generic-syntax-accepted.sh && docs/tests/m0012-type-generic-syntax-concrete-draft.sh && docs/tests/m0012-type-generic-syntax-review.sh && docs/tests/m0012-type-generic-syntax-proposal.sh && docs/tests/m0012-type-generic-parser-blocked.sh && docs/tests/m0011-declaration-parser-implementation.sh && docs/tests/m0011-declaration-ast-shell.sh && docs/tests/m0011-declaration-parser-fixtures.sh && docs/tests/m0011-declaration-syntax-accepted.sh && docs/tests/m0010-parser-recovery-architecture.sh && docs/tests/m0009-ast-data-model.sh && docs/tests/m0008-grammar-authority-ledger.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0012-type-generic-syntax-accepted.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
  - `docs/SPEC.md`
  - `docs/ambiguities/M0008-type-generic-syntax.md`
  - `docs/syntax/grammar-authority-ledger.md`
  - `docs/adr/proposals/reviews/ADR-0023-chief-architect-decision.md`
  - M0012 validators and task metadata

## Forbidden Changes

- Do not add type parser APIs, type AST nodes, HIR, MIR, or type/generic fixtures.
- Do not implement type checking or capability analysis.
- Do not weaken or delete failing tests without main-task review approval.
- Do not introduce language semantics outside ADR-0023.

## Ambiguities And Dependencies

- Expression, statement, pattern, coroutine, and unsafe syntax remain ambiguous for M0013.

## Execution Log

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created ADR-0023 acceptance task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Created accepted-state validator before accepting ADR-0023.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=Tests fail before implementation for the expected reason: accepted ADR-0023 did not exist.
2026-07-10 main_task=Chief-Architect phase=implementation result=pass notes=Promoted reviewed concrete draft to accepted ADR-0023, updated SPEC, resolved ambiguity, updated ledger, and approved decision artifact without parser implementation.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=Focused M0012 and M0008 validators pass for accepted authority and deferred implementation.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=Acceptance keeps parser APIs, type AST nodes, and type/generic parser fixtures absent.
2026-07-10 main_task=main-task review phase=review result=pass notes=Review compares accepted ADR-0023, SPEC summary, ambiguity resolution, and milestone scope.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=CI passes as final gate.
```

## Handoff

- Next main task: `main-task test work`
- Reason: `Create concrete type/generic fixtures after accepted source-of-truth update.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
  - `docs/milestones/M0012-type-and-generic-syntax-parser.md`
