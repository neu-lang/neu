# Task: M0016-003 Review name resolution policy proposal

## Task Metadata

- Task ID: `M0016-003`
- Milestone: `M0016`
- Milestone File: `docs/milestones/M0016-name-resolution-pass.md`
- Status: `complete`
- Owner main task: `main-task language review`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0016-003-name-resolution-policy-proposal-review`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/proposals/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
  - `docs/adr/ADR-0022-declaration-syntax.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Review ADR-0026 as a non-authoritative name-resolution policy proposal and identify what must be revised before acceptance.

## Motivation

ADR-0026 proposes a direction but still leaves concrete accepted implementation rules unresolved. Review must prevent implementation from depending on it and must identify missing decisions that block M0016.

## Scope

- Add main-task language review review for semantic consistency and missing authority.
- Add main-task diagnostics check review for resolution diagnostic obligations.
- Add main-task adversarial check review for soundness and ambiguity risks.
- Add main-task specification check review for source-of-truth boundaries.
- Add main-task simplicity check review for bootstrap subset control.
- Add main task decision artifact preserving pending status.

## Out Of Scope

- Revising ADR-0026.
- Accepting ADR-0026.
- Changing `docs/SPEC.md`.
- Creating resolution fixtures.
- Adding name-resolution implementation.
- Resolving `docs/ambiguities/M0016-name-resolution-policy.md`.

## Required Inputs

- Milestone: `docs/milestones/M0016-name-resolution-pass.md`
- Proposal: `docs/adr/proposals/ADR-0026-name-resolution-policy.md`
- Ambiguity report: `docs/ambiguities/M0016-name-resolution-policy.md`
- Accepted ADRs listed in Source Of Truth.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - All required review artifacts exist and request revision before acceptance.
- Negative tests:
  - No accepted ADR-0026, spec section, name-resolution implementation, or resolution fixtures are added.
- Diagnostic tests:
  - Reviews request concrete diagnostics with primary span, recovery action, source-of-truth citation, and safe suggestion policy.
- Adversarial tests:
  - Reviews keep M0016 blocked and reject semantic inference from proposal text.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0016-name-resolution-policy-review.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - ADR-0026 review artifacts do not exist yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Create review artifacts only. Keep ADR-0026 as a draft proposal and leave the ambiguity open.

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
- [x] M0016 remains blocked pending accepted source-of-truth authority.

## Execution Commands

- Generate tests: `create docs/tests/m0016-name-resolution-policy-review.sh`
- Verify tests fail: `docs/tests/m0016-name-resolution-policy-review.sh`
- Ordinary tests: `docs/tests/m0016-name-resolution-policy-review.sh && docs/tests/m0016-name-resolution-policy-proposal.sh && docs/tests/m0016-name-resolution-blocked.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0016-003-name-resolution-policy-proposal-review.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0016-003-name-resolution-policy-proposal-review.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0016-name-resolution-policy-review.sh && docs/tests/m0016-name-resolution-policy-proposal.sh && docs/tests/m0016-name-resolution-blocked.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0016-name-resolution-policy-review.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/adr/proposals/reviews/ADR-0026-language-lawyer-review.md`
  - `docs/adr/proposals/reviews/ADR-0026-diagnostics-review.md`
  - `docs/adr/proposals/reviews/ADR-0026-adversarial-review.md`
  - `docs/adr/proposals/reviews/ADR-0026-spec-compliance-review.md`
  - `docs/adr/proposals/reviews/ADR-0026-simplicity-review.md`
  - `docs/adr/proposals/reviews/ADR-0026-chief-architect-decision.md`
  - `docs/tasks/M0016-003-name-resolution-policy-proposal-review.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted ADRs under `docs/adr/`.
- Do not close `docs/ambiguities/M0016-name-resolution-policy.md`.
- Do not accept ADR-0026.
- Do not implement name resolution.
- Do not implement resolution diagnostics.

## Ambiguities And Dependencies

- ADR-0026 remains a proposal.
- M0016 remains blocked until accepted source of truth exists.

## Execution Log

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0016 ADR-0026 review task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0016-name-resolution-policy-review.sh before adding ADR-0026 review artifacts.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=Review validator failed before implementation because ADR-0026 review artifacts did not exist.
2026-07-10 main_task=Language-Lawyer phase=implementation result=pass notes=Added required ADR-0026 review artifacts and kept ADR-0026 pending revision.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=docs/tests/m0016-name-resolution-policy-review.sh, docs/tests/m0016-name-resolution-policy-proposal.sh, and docs/tests/m0016-name-resolution-blocked.sh passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0016-003-soundness.md after ordinary tests were recorded.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0016-003-review.md approved review-only scope against docs/SPEC.md and docs/milestones/M0016-name-resolution-pass.md.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt, cargo clippy, cargo test, M0016 review/proposal/blocker validators, M0015 validator, and M0002 validator passed.
```

## Handoff

- Next main task: `main-task language review`
- Reason: `Audit ADR-0026 before acceptance.`
- Required Context:
  - This task file
  - `docs/adr/proposals/ADR-0026-name-resolution-policy.md`
  - `docs/ambiguities/M0016-name-resolution-policy.md`
  - `docs/milestones/M0016-name-resolution-pass.md`
