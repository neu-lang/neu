# Task: M0012-004 Revise ADR-0023 with concrete draft grammar

## Task Metadata

- Task ID: `M0012-004`
- Milestone: `M0012`
- Milestone File: `docs/milestones/M0012-type-and-generic-syntax-parser.md`
- Status: `complete`
- Owner main task: `main-task semantic design`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0012-004-type-generic-syntax-concrete-draft`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0016-generics-and-parametric-polymorphism.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Revise the non-authoritative ADR-0023 proposal to include concrete draft grammar, binding rules, recovery boundaries, diagnostics, and explicit deferrals required by review.

## Motivation

ADR-0023 reviews request revision before acceptance because the proposal lacks concrete grammar. This task makes the proposal reviewable for final acceptance while keeping it draft-only.

## Scope

- Add concrete draft grammar sections to `docs/adr/proposals/ADR-0023-type-and-generic-syntax.md`.
- Specify nullable binding, generic parameter placement, generic argument grammar, capability-bound syntax, function type grammar, grouping, recovery, diagnostics, and deferrals.
- Resolve the review attack examples in draft form.
- Keep M0012 ambiguity open and main task decision pending.

## Out Of Scope

- Accepting ADR-0023.
- Updating `docs/SPEC.md`.
- Updating accepted ADRs under `docs/adr/`.
- Adding type parser implementation.
- Adding type AST nodes or type/generic parser fixtures.
- Closing `docs/ambiguities/M0008-type-generic-syntax.md`.

## Required Inputs

- Milestone: `docs/milestones/M0012-type-and-generic-syntax-parser.md`
- ADR proposal: `docs/adr/proposals/ADR-0023-type-and-generic-syntax.md`
- Review files:
  - `docs/adr/proposals/reviews/ADR-0023-language-lawyer-review.md`
  - `docs/adr/proposals/reviews/ADR-0023-adversarial-review.md`
  - `docs/adr/proposals/reviews/ADR-0023-diagnostics-review.md`
  - `docs/adr/proposals/reviews/ADR-0023-simplicity-review.md`
  - `docs/adr/proposals/reviews/ADR-0023-chief-architect-decision.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Proposal contains concrete draft grammar sections and required diagnostics.
- Negative tests:
  - Accepted ADR-0023, SPEC update, parser type APIs, type AST nodes, and type/generic fixtures remain absent.
- Diagnostic tests:
  - Draft diagnostics include primary span and recovery action obligations.
- Adversarial tests:
  - Draft addresses `Box<T?>?`, `fun f<T: Send, Share>();`, `(T) -> U?`, and `((T) -> U)?`.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0012-type-generic-syntax-concrete-draft.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - ADR-0023 does not yet contain `## Concrete Draft Grammar`.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Revise the draft proposal only. Do not move the draft into accepted ADRs or modify parser behavior.

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
- [x] M0012 remains blocked pending accepted syntax authority.

## Execution Commands

- Generate tests: `create docs/tests/m0012-type-generic-syntax-concrete-draft.sh`
- Verify tests fail: `docs/tests/m0012-type-generic-syntax-concrete-draft.sh`
- Ordinary tests: `docs/tests/m0012-type-generic-syntax-concrete-draft.sh && docs/tests/m0012-type-generic-syntax-review.sh && docs/tests/m0012-type-generic-syntax-proposal.sh && docs/tests/m0012-type-generic-parser-blocked.sh`
- Adversarial tests: `docs/tests/m0012-type-generic-syntax-concrete-draft.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0012-004-type-generic-syntax-concrete-draft.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0012-type-generic-syntax-concrete-draft.sh && docs/tests/m0012-type-generic-syntax-review.sh && docs/tests/m0012-type-generic-syntax-proposal.sh && docs/tests/m0012-type-generic-parser-blocked.sh && docs/tests/m0011-declaration-parser-implementation.sh && docs/tests/m0011-declaration-ast-shell.sh && docs/tests/m0011-declaration-parser-fixtures.sh && docs/tests/m0011-declaration-syntax-accepted.sh && docs/tests/m0010-parser-recovery-architecture.sh && docs/tests/m0009-ast-data-model.sh && docs/tests/m0008-grammar-authority-ledger.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0012-type-generic-syntax-concrete-draft.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/adr/proposals/ADR-0023-type-and-generic-syntax.md`
  - `docs/tasks/M0012-004-type-generic-syntax-concrete-draft.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted ADRs under `docs/adr/`.
- Do not accept ADR-0023.
- Do not close `docs/ambiguities/M0008-type-generic-syntax.md`.
- Do not add type parser APIs, type AST nodes, or type/generic fixtures.

## Ambiguities And Dependencies

- ADR-0023 remains draft-only until accepted by main task.
- M0012 remains blocked by `docs/ambiguities/M0008-type-generic-syntax.md`.

## Execution Log

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created ADR-0023 concrete draft revision task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Created concrete draft validator before revising proposal.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=docs/tests/m0012-type-generic-syntax-concrete-draft.sh failed because ADR-0023 lacked Concrete Draft Grammar.
2026-07-10 main_task=Language-Designer phase=implementation result=pass notes=Revised ADR-0023 with concrete draft grammar, binding examples, recovery boundaries, diagnostics, and deferrals.
2026-07-10 main_task=Language-Designer phase=ordinary-tests result=pass notes=M0012 concrete draft, review, proposal, and blocker validators passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0012-004-soundness.md after ordinary-test evidence.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0012-004-review.md approves concrete draft and preserves M0012 blocker.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && M0012-M0002 validation scripts passed.
```

## Handoff

- Next main task: `main task`
- Reason: `Consider whether concrete ADR-0023 draft can be accepted after validation.`
- Required Context:
  - This task file
  - `docs/adr/proposals/ADR-0023-type-and-generic-syntax.md`
  - ADR-0023 review files
