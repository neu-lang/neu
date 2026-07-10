# Task: M0019-005 Accept Nullability And Flow Typing ADR

## Task Metadata

- Task ID: `M0019-005`
- Milestone: `M0019`
- Milestone File: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Status: `complete`
- Owner main task: `main task`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0019-005-accept-nullability-flow-adr`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md`
  - `docs/adr/proposals/reviews/ADR-0028-chief-architect-decision.md`
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0011-flow-typing-and-smart-casts.md`
  - `docs/adr/ADR-0013-mutability-model.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/ADR-0027-type-checking-core.md`
- Ambiguity: `docs/ambiguities/M0019-nullability-and-flow-typing.md`
- Milestone: `docs/milestones/M0019-nullability-and-flow-typing.md`

## Goal

Accept ADR-0028 as the M0019 nullability and flow-typing source of truth, resolve the M0019 ambiguity, and unblock implementation tasks without implementing flow typing.

## Motivation

ADR-0028 has been drafted, reviewed, and revised into a concrete model. M0019 implementation must not proceed until accepted source of truth defines null-test recognition, branch region boundaries, refined output shape, nullable-use diagnostics, and explicit deferrals.

## Scope

- Create accepted `docs/adr/ADR-0028-nullability-and-flow-typing.md`.
- Update `docs/SPEC.md` with an ADR-0028 summary.
- Resolve `docs/ambiguities/M0019-nullability-and-flow-typing.md`.
- Update `docs/adr/proposals/reviews/ADR-0028-chief-architect-decision.md` to approved.
- Add accepted-state validator for ADR-0028.

## Out Of Scope

- Implementing nullability checks.
- Implementing flow refinement tracking.
- Adding compiler fixtures for M0019 behavior.
- Changing parser behavior.
- Adding HIR, MIR, ownership, borrow, coroutine, unsafe, FFI, or backend behavior.

## Required Inputs

- Proposal: `docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md`
- Reviews: `docs/adr/proposals/reviews/ADR-0028-*.md`
- Ambiguity: `docs/ambiguities/M0019-nullability-and-flow-typing.md`
- Milestone: `docs/milestones/M0019-nullability-and-flow-typing.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Accepted ADR-0028 exists and has `Status: Accepted`.
  - `docs/SPEC.md` contains ADR-0028 semantics.
  - M0019 ambiguity report is resolved.
  - main task decision is approved.
- Negative tests:
  - Compiler flow implementation remains absent.
  - Acceptance does not activate members, calls, aliases, coroutine suspension, unsafe, FFI, generics, patterns, exclusive borrows, HIR, MIR, or backend behavior.
- Diagnostic tests:
  - Accepted ADR diagnostic obligations include primary span, recovery action, source-of-truth citation, safe suggestion policy, and stable rule identifiers.
- Adversarial tests:
  - Acceptance does not broaden beyond the reviewed nullability and local immutable flow subset.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0019-nullability-flow-accepted.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Accepted `docs/adr/ADR-0028-nullability-and-flow-typing.md` does not exist yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Promote the reviewed concrete proposal into an accepted ADR, add the corresponding SPEC summary, resolve the ambiguity, approve the main task decision, and leave implementation for later tasks.

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
- [x] M0019 ambiguity is resolved by accepted source of truth.

## Execution Commands

- Generate tests: `create docs/tests/m0019-nullability-flow-accepted.sh`
- Verify tests fail: `sh docs/tests/m0019-nullability-flow-accepted.sh`
- Ordinary tests: `sh docs/tests/m0019-nullability-flow-accepted.sh && sh docs/tests/m0019-nullability-flow-concrete-draft.sh && sh docs/tests/m0019-nullability-flow-review.sh && sh docs/tests/m0019-nullability-flow-proposal.sh && sh docs/tests/m0019-nullability-flow-blocked.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0019-005-accept-nullability-flow-adr.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0019-005-accept-nullability-flow-adr.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0019-nullability-flow-accepted.sh && sh docs/tests/m0019-nullability-flow-concrete-draft.sh && sh docs/tests/m0019-nullability-flow-review.sh && sh docs/tests/m0019-nullability-flow-proposal.sh && sh docs/tests/m0019-nullability-flow-blocked.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0019-nullability-flow-accepted.sh`
  - `docs/tests/m0019-nullability-flow-concrete-draft.sh`
  - `docs/tests/m0019-nullability-flow-review.sh`
  - `docs/tests/m0019-nullability-flow-proposal.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
  - `docs/SPEC.md`
  - `docs/ambiguities/M0019-nullability-and-flow-typing.md`
  - `docs/adr/proposals/reviews/ADR-0028-chief-architect-decision.md`
  - `docs/tasks/M0019-005-accept-nullability-flow-adr.md`
  - `docs/tasks/reviews/M0019-005-review.md`
  - `docs/tasks/soundness/M0019-005-soundness.md`

## Forbidden Changes

- Do not implement nullability checks.
- Do not implement flow refinement tracking.
- Do not add compiler fixtures for M0019 behavior.
- Do not add HIR, MIR, ownership, borrow, coroutine, unsafe, FFI, or backend behavior.
- Do not weaken or delete existing M0019 validators to hide missing accepted semantics.

## Ambiguities And Dependencies

- M0019 implementation remains dependent on this task until accepted ADR-0028 and SPEC updates are present.
- Members, calls, aliases, coroutine suspension, unsafe, FFI, generics, patterns, exclusive borrows, HIR, MIR, and backend behavior remain deferred after this task.

## Execution Log

- 2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created ADR-0028 acceptance task.
- 2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0019-nullability-flow-accepted.sh before accepting ADR-0028.
- 2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=Accepted-state validator failed before implementation because docs/adr/ADR-0028-nullability-and-flow-typing.md was missing.
- 2026-07-10 main_task=Chief-Architect phase=implementation result=pass notes=Accepted ADR-0028, updated SPEC, resolved M0019 ambiguity, approved main task decision, and aligned M0019 validators for accepted state.
- 2026-07-10 main_task=Chief-Architect phase=ordinary-tests result=pass notes=`sh docs/tests/m0019-nullability-flow-accepted.sh`, `sh docs/tests/m0019-nullability-flow-concrete-draft.sh`, `sh docs/tests/m0019-nullability-flow-review.sh`, `sh docs/tests/m0019-nullability-flow-proposal.sh`, and `sh docs/tests/m0019-nullability-flow-blocked.sh` passed.
- 2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0019-005-accept-nullability-flow-adr.md` created a passing soundness report.
- 2026-07-10 main_task=main-task review phase=review result=pass notes=`docs/scripts/review-task.sh docs/tasks/M0019-005-accept-nullability-flow-adr.md` created review report; concrete review approved acceptance-only scope.
- 2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, `sh docs/tests/m0019-nullability-flow-accepted.sh`, `sh docs/tests/m0019-nullability-flow-concrete-draft.sh`, `sh docs/tests/m0019-nullability-flow-review.sh`, `sh docs/tests/m0019-nullability-flow-proposal.sh`, `sh docs/tests/m0019-nullability-flow-blocked.sh`, and `sh docs/tests/m0002-workspace-ci.sh` passed.

## Handoff

- Next main task: `main-task test work`
- Reason: `Verify accepted-state validator fails before accepted ADR-0028 exists.`
- Required Context:
  - This task file
  - `docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md`
  - `docs/adr/proposals/reviews/ADR-0028-*.md`
  - `docs/ambiguities/M0019-nullability-and-flow-typing.md`
