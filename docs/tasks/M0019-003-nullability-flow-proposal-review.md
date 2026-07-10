# Task: M0019-003 Review Nullability And Flow Typing Proposal

## Task Metadata

- Task ID: `M0019-003`
- Milestone: `M0019`
- Milestone File: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Status: `complete`
- Owner main task: `main-task language review`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0019-003-nullability-flow-proposal-review`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md`
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0011-flow-typing-and-smart-casts.md`
  - `docs/adr/ADR-0013-mutability-model.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/ADR-0027-type-checking-core.md`
- Milestone: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Ambiguity: `docs/ambiguities/M0019-nullability-and-flow-typing.md`

## Goal

Review ADR-0028 as a non-authoritative nullability and flow-typing proposal and identify revisions required before acceptance.

## Motivation

M0019 cannot implement nullability checks, flow refinement tracking, invalidation, or diagnostics until accepted source of truth defines concrete rules. ADR-0028 is a draft and needs specialty review before acceptance.

## Scope

- Add main-task language review review.
- Add main-task diagnostics check review.
- Add main-task adversarial check review.
- Add main-task specification check review.
- Add main-task simplicity check review.
- Add main task decision artifact preserving pending status.
- Add validator for review completeness.

## Out Of Scope

- Revising ADR-0028.
- Accepting ADR-0028.
- Changing `docs/SPEC.md`.
- Implementing nullability checks.
- Implementing flow refinement tracking.
- Resolving `docs/ambiguities/M0019-nullability-and-flow-typing.md`.

## Required Inputs

- Proposal: `docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md`
- Ambiguity: `docs/ambiguities/M0019-nullability-and-flow-typing.md`
- Milestone: `docs/milestones/M0019-nullability-and-flow-typing.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Required review artifacts exist and require revision before acceptance.
- Negative tests:
  - Accepted ADR-0028 remains absent.
  - `docs/SPEC.md` does not contain ADR-0028.
  - The M0019 ambiguity report remains open.
- Diagnostic tests:
  - Reviews require concrete diagnostic primary spans, recovery actions, source-of-truth citations, and safe suggestion policies.
- Adversarial tests:
  - Reviews keep M0019 blocked and reject implementation based on draft text.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0019-nullability-flow-review.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - ADR-0028 review artifacts do not exist.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Create review artifacts only. Keep ADR-0028 as a draft proposal and leave M0019 ambiguity open.

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
- [x] M0019 remains blocked pending accepted source-of-truth authority.

## Execution Commands

- Generate tests: `create docs/tests/m0019-nullability-flow-review.sh`
- Verify tests fail: `sh docs/tests/m0019-nullability-flow-review.sh`
- Ordinary tests: `sh docs/tests/m0019-nullability-flow-review.sh && sh docs/tests/m0019-nullability-flow-proposal.sh && sh docs/tests/m0019-nullability-flow-blocked.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0019-003-nullability-flow-proposal-review.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0019-003-nullability-flow-proposal-review.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0019-nullability-flow-review.sh && sh docs/tests/m0019-nullability-flow-proposal.sh && sh docs/tests/m0019-nullability-flow-blocked.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0019-nullability-flow-review.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/adr/proposals/reviews/ADR-0028-language-lawyer-review.md`
  - `docs/adr/proposals/reviews/ADR-0028-diagnostics-review.md`
  - `docs/adr/proposals/reviews/ADR-0028-adversarial-review.md`
  - `docs/adr/proposals/reviews/ADR-0028-spec-compliance-review.md`
  - `docs/adr/proposals/reviews/ADR-0028-simplicity-review.md`
  - `docs/adr/proposals/reviews/ADR-0028-chief-architect-decision.md`
  - `docs/tasks/M0019-003-nullability-flow-proposal-review.md`
  - `docs/tasks/reviews/M0019-003-review.md`
  - `docs/tasks/soundness/M0019-003-soundness.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted ADRs under `docs/adr/`.
- Do not close `docs/ambiguities/M0019-nullability-and-flow-typing.md`.
- Do not accept ADR-0028.
- Do not implement nullability checks.
- Do not implement flow refinement tracking.
- Do not add examples for unaccepted semantics.

## Ambiguities And Dependencies

- ADR-0028 remains a proposal.
- M0019 remains blocked until accepted source of truth exists.

## Execution Log

- 2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0019 ADR-0028 review task.
- 2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0019-nullability-flow-review.sh before adding ADR-0028 review artifacts.
- 2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=Review validator failed before implementation because ADR-0028 review artifacts were missing.
- 2026-07-10 main_task=Language-Lawyer phase=implementation result=pass notes=Added ADR-0028 specialty review artifacts; proposal remains non-authoritative and M0019 ambiguity remains open.
- 2026-07-10 main_task=Language-Lawyer phase=ordinary-tests result=pass notes=`sh docs/tests/m0019-nullability-flow-review.sh`, `sh docs/tests/m0019-nullability-flow-proposal.sh`, and `sh docs/tests/m0019-nullability-flow-blocked.sh` passed.
- 2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0019-003-nullability-flow-proposal-review.md` created a passing soundness report.
- 2026-07-10 main_task=main-task review phase=review result=pass notes=`docs/scripts/review-task.sh docs/tasks/M0019-003-nullability-flow-proposal-review.md` created review report; concrete review approved review-only scope.
- 2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, `sh docs/tests/m0019-nullability-flow-review.sh`, `sh docs/tests/m0019-nullability-flow-proposal.sh`, `sh docs/tests/m0019-nullability-flow-blocked.sh`, and `sh docs/tests/m0002-workspace-ci.sh` passed.

## Handoff

- Next main task: `main-task test work`
- Reason: `Verify the review validator fails before review artifacts exist.`
- Required Context:
  - This task file
  - `docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md`
  - `docs/ambiguities/M0019-nullability-and-flow-typing.md`
  - `docs/milestones/M0019-nullability-and-flow-typing.md`
