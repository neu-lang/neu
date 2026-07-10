# Task: M0012-002 Draft type and generic syntax proposal

## Task Metadata

- Task ID: `M0012-002`
- Milestone: `M0012`
- Milestone File: `docs/milestones/M0012-type-and-generic-syntax-parser.md`
- Status: `complete`
- Owner main task: `main-task semantic design`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0012-002-type-generic-syntax-proposal`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0016-generics-and-parametric-polymorphism.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Draft a non-authoritative ADR proposal for type, nullable, generic, function-type, and capability-bound syntax sufficient for later M0012 review.

## Motivation

M0012 is blocked because accepted ADRs define type-system semantics but not concrete parser grammar. A proposal is needed before review and acceptance can unblock type parser fixtures or implementation.

## Scope

- Add `docs/adr/proposals/ADR-0023-type-and-generic-syntax.md`.
- Explain competing designs and trade-offs.
- Recommend a draft syntax direction.
- Enumerate required accepted content before parser implementation.
- State explicit non-authority notice and implementation prohibition.
- Preserve `docs/ambiguities/M0008-type-generic-syntax.md` as open.

## Out Of Scope

- Accepting ADR-0023.
- Updating `docs/SPEC.md`.
- Updating accepted ADRs under `docs/adr/`.
- Adding type parser implementation.
- Adding concrete type/generic fixtures.
- Resolving the M0012 ambiguity.

## Required Inputs

- Milestone: `docs/milestones/M0012-type-and-generic-syntax-parser.md`
- Spec sections:
  - `ADR-0006: Nullability And Absence`
  - `ADR-0010: Type System Shape`
  - `ADR-0016: Generics And Parametric Polymorphism`
- ADRs:
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0016-generics-and-parametric-polymorphism.md`
- Existing files:
  - `docs/ambiguities/M0008-type-generic-syntax.md`
  - `docs/syntax/grammar-authority-ledger.md`
  - `docs/tasks/M0012-001-type-generic-syntax-blocker.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Proposal file exists with non-authority notice, competing designs, recommended draft choice, required accepted content, downstream consequences, and dependencies.
- Negative tests:
  - Accepted ADR-0023 does not exist.
  - `docs/SPEC.md` is not updated.
  - Type parser APIs, type AST nodes, and type/generic fixtures remain absent.
- Diagnostic tests:
  - Proposal names required diagnostic obligations for future accepted syntax.
- Adversarial tests:
  - Proposal must not infer Kotlin, Rust, or existing compiler behavior as authority.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0012-type-generic-syntax-proposal.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `docs/adr/proposals/ADR-0023-type-and-generic-syntax.md` does not exist.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a draft proposal only. Do not move the draft into accepted ADRs or modify parser behavior.

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

- Generate tests: `create docs/tests/m0012-type-generic-syntax-proposal.sh`
- Verify tests fail: `docs/tests/m0012-type-generic-syntax-proposal.sh`
- Ordinary tests: `docs/tests/m0012-type-generic-syntax-proposal.sh && docs/tests/m0012-type-generic-parser-blocked.sh`
- Adversarial tests: `docs/tests/m0012-type-generic-syntax-proposal.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0012-002-type-generic-syntax-proposal.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0012-type-generic-syntax-proposal.sh && docs/tests/m0012-type-generic-parser-blocked.sh && docs/tests/m0011-declaration-parser-implementation.sh && docs/tests/m0011-declaration-ast-shell.sh && docs/tests/m0011-declaration-parser-fixtures.sh && docs/tests/m0011-declaration-syntax-accepted.sh && docs/tests/m0010-parser-recovery-architecture.sh && docs/tests/m0009-ast-data-model.sh && docs/tests/m0008-grammar-authority-ledger.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0012-type-generic-syntax-proposal.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/adr/proposals/ADR-0023-type-and-generic-syntax.md`
  - `docs/tasks/M0012-002-type-generic-syntax-proposal.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not close `docs/ambiguities/M0008-type-generic-syntax.md`.
- Do not add type parser APIs, type AST nodes, or type/generic fixtures.
- Do not treat the proposal as source of truth.

## Ambiguities And Dependencies

- M0012 remains blocked by `docs/ambiguities/M0008-type-generic-syntax.md`.
- Required follow-up reviews: main-task language review, main-task adversarial check, main-task diagnostics check, main-task simplicity check, main task.

## Execution Log

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0012 type/generic syntax proposal task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Created proposal validator before adding proposal file.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=docs/tests/m0012-type-generic-syntax-proposal.sh failed because ADR-0023 proposal file was missing.
2026-07-10 main_task=Language-Designer phase=implementation result=pass notes=Added non-authoritative ADR-0023 type and generic syntax draft proposal.
2026-07-10 main_task=Language-Designer phase=ordinary-tests result=pass notes=docs/tests/m0012-type-generic-syntax-proposal.sh and docs/tests/m0012-type-generic-parser-blocked.sh passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0012-002-soundness.md after ordinary-test evidence.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0012-002-review.md approves the non-authoritative proposal and preserves M0012 blocker.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && M0012-M0002 validation scripts passed.
```

## Handoff

- Next main task: `main-task language review`
- Reason: `Audit the draft before acceptance.`
- Required Context:
  - This task file
  - `docs/adr/proposals/ADR-0023-type-and-generic-syntax.md`
  - `docs/ambiguities/M0008-type-generic-syntax.md`
  - `docs/syntax/grammar-authority-ledger.md`
