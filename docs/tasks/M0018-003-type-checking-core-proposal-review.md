# Task: M0018-003 Review Type Checking Core Proposal

## Task Metadata

- Task ID: `M0018-003`
- Milestone: `M0018`
- Milestone File: `docs/milestones/M0018-type-checking-core.md`
- Status: `complete`
- Owner main task: `main-task language review`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0018-003-type-checking-core-proposal-review`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/proposals/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/ADR-0026-name-resolution-policy.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`
- Ambiguity: `docs/ambiguities/M0018-type-checking-core.md`

## Goal

Review ADR-0027 as a non-authoritative type-checking core proposal and identify revisions required before acceptance.

## Motivation

M0018 cannot implement well-typed or ill-typed fixtures until accepted source of truth defines primitive categories, literal typing, assignment compatibility, calls, function type application, typed output, and diagnostics.

## Scope

- Add main-task language review review.
- Add main-task diagnostics check review.
- Add main-task adversarial check review.
- Add main-task specification check review.
- Add main-task simplicity check review.
- Add main task decision artifact preserving pending status.
- Add validator for review completeness.

## Out Of Scope

- Revising ADR-0027.
- Accepting ADR-0027.
- Changing `docs/SPEC.md`.
- Implementing type checking.
- Resolving `docs/ambiguities/M0018-type-checking-core.md`.

## Required Inputs

- Proposal: `docs/adr/proposals/ADR-0027-type-checking-core.md`
- Ambiguity: `docs/ambiguities/M0018-type-checking-core.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Required review artifacts exist and require revision before acceptance.
- Negative tests:
  - Accepted ADR-0027 remains absent.
  - `docs/SPEC.md` does not contain ADR-0027.
  - Type-check implementation remains diagnostic-only.
- Diagnostic tests:
  - Reviews require concrete diagnostic primary spans, recovery actions, source-of-truth citations, and safe suggestion policies.
- Adversarial tests:
  - Reviews keep M0018 blocked and reject implementation based on draft text.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0018-type-checking-core-review.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - ADR-0027 review artifacts do not exist.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Create review artifacts only. Keep ADR-0027 as a draft proposal and leave M0018 ambiguity open.

## Acceptance Criteria

- [ ] Task references exactly one milestone.
- [ ] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing change.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] main-task review compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [x] M0018 remains blocked pending accepted source-of-truth authority.

## Execution Commands

- Generate tests: `create docs/tests/m0018-type-checking-core-review.sh`
- Verify tests fail: `sh docs/tests/m0018-type-checking-core-review.sh`
- Ordinary tests: `sh docs/tests/m0018-type-checking-core-review.sh && sh docs/tests/m0018-type-checking-core-proposal.sh && sh docs/tests/m0018-type-checking-ambiguity-blocker.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0018-003-type-checking-core-proposal-review.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0018-003-type-checking-core-proposal-review.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0018-type-checking-core-review.sh && sh docs/tests/m0018-type-checking-core-proposal.sh && sh docs/tests/m0018-type-checking-ambiguity-blocker.sh && sh docs/tests/m0017-unsupported-type-form-blocking.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0018-type-checking-core-review.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/adr/proposals/reviews/ADR-0027-language-lawyer-review.md`
  - `docs/adr/proposals/reviews/ADR-0027-diagnostics-review.md`
  - `docs/adr/proposals/reviews/ADR-0027-adversarial-review.md`
  - `docs/adr/proposals/reviews/ADR-0027-spec-compliance-review.md`
  - `docs/adr/proposals/reviews/ADR-0027-simplicity-review.md`
  - `docs/adr/proposals/reviews/ADR-0027-chief-architect-decision.md`
  - `docs/tasks/M0018-003-type-checking-core-proposal-review.md`
  - `docs/tasks/reviews/M0018-003-review.md`
  - `docs/tasks/soundness/M0018-003-soundness.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted ADRs under `docs/adr/`.
- Do not close `docs/ambiguities/M0018-type-checking-core.md`.
- Do not accept ADR-0027.
- Do not implement type checking.

## Ambiguities And Dependencies

- ADR-0027 remains a proposal.
- M0018 remains blocked until accepted source of truth exists.

## Execution Log

- 2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0018 ADR-0027 review task.
- 2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0018-type-checking-core-review.sh before adding ADR-0027 review artifacts.
- 2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=Review validator failed before implementation because ADR-0027 review artifacts were missing.
- 2026-07-10 main_task=Language-Lawyer phase=ordinary-tests result=pass notes=Added ADR-0027 specialty review artifacts; M0018 review, proposal, and ambiguity blocker validators passed.
- 2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0018-003-type-checking-core-proposal-review.md` created a passing soundness report.
- 2026-07-10 main_task=main-task review phase=review result=pass notes=`docs/scripts/review-task.sh docs/tasks/M0018-003-type-checking-core-proposal-review.md` created review and concrete review approved review-only scope.
- 2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, `sh docs/tests/m0018-type-checking-core-review.sh`, `sh docs/tests/m0018-type-checking-core-proposal.sh`, `sh docs/tests/m0018-type-checking-ambiguity-blocker.sh`, and `sh docs/tests/m0002-workspace-ci.sh` passed.
