# Task: M0018-005 Accept Type Checking Core ADR

## Task Metadata

- Task ID: `M0018-005`
- Milestone: `M0018`
- Milestone File: `docs/milestones/M0018-type-checking-core.md`
- Status: `complete`
- Owner main task: `main task`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0018-005-accept-type-checking-core-adr`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/proposals/ADR-0027-type-checking-core.md`
  - `docs/adr/proposals/reviews/ADR-0027-chief-architect-decision.md`
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/ADR-0026-name-resolution-policy.md`
- Ambiguity: `docs/ambiguities/M0018-type-checking-core.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`

## Goal

Accept ADR-0027 as the M0018 type-checking core source of truth, resolve the M0018 ambiguity, and unblock implementation tasks without implementing type checking.

## Motivation

ADR-0027 has been drafted, reviewed, and revised into a concrete bootstrap model. M0018 implementation must not proceed until accepted source of truth defines typed output, primitive identities, literals, assignment compatibility, diagnostics, and deferrals.

## Scope

- Create accepted `docs/adr/ADR-0027-type-checking-core.md`.
- Update `docs/SPEC.md` with an ADR-0027 summary.
- Resolve `docs/ambiguities/M0018-type-checking-core.md`.
- Update `docs/adr/proposals/reviews/ADR-0027-chief-architect-decision.md` to approved.
- Add accepted-state validator for ADR-0027.

## Out Of Scope

- Implementing type checking.
- Adding well-typed or ill-typed fixtures.
- Adding primitive type records.
- Changing parser behavior.
- Adding HIR, MIR, ownership, borrow, or backend behavior.

## Required Inputs

- Proposal: `docs/adr/proposals/ADR-0027-type-checking-core.md`
- Reviews: `docs/adr/proposals/reviews/ADR-0027-*.md`
- Ambiguity: `docs/ambiguities/M0018-type-checking-core.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Accepted ADR-0027 exists and has `Status: Accepted`.
  - `docs/SPEC.md` contains ADR-0027 semantics.
  - M0018 ambiguity report is resolved.
  - main task decision is approved.
- Negative tests:
  - Compiler type-checking implementation remains diagnostic-only.
  - Accepted ADR does not activate direct calls, structural function type application, overloads, numeric conversion, member lookup, generic solving, ownership, borrow checking, HIR, MIR, or backend behavior.
- Diagnostic tests:
  - Accepted ADR diagnostic obligations include primary span, recovery action, source-of-truth citation, safe suggestion policy, and stable rule identifiers.
- Adversarial tests:
  - Acceptance does not broaden beyond the reviewed bootstrap subset.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0018-type-checking-core-accepted.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Accepted `docs/adr/ADR-0027-type-checking-core.md` does not exist yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Promote the reviewed concrete proposal into an accepted ADR, add the corresponding SPEC summary, resolve the ambiguity, approve the main task decision, and leave implementation for later tasks.

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
- [x] M0018 ambiguity is resolved by accepted source of truth.

## Execution Commands

- Generate tests: `create docs/tests/m0018-type-checking-core-accepted.sh`
- Verify tests fail: `sh docs/tests/m0018-type-checking-core-accepted.sh`
- Ordinary tests: `sh docs/tests/m0018-type-checking-core-accepted.sh && sh docs/tests/m0018-type-checking-core-concrete-draft.sh && sh docs/tests/m0018-type-checking-core-review.sh && sh docs/tests/m0018-type-checking-core-proposal.sh && sh docs/tests/m0018-type-checking-ambiguity-blocker.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0018-005-accept-type-checking-core-adr.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0018-005-accept-type-checking-core-adr.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0018-type-checking-core-accepted.sh && sh docs/tests/m0018-type-checking-core-concrete-draft.sh && sh docs/tests/m0018-type-checking-core-review.sh && sh docs/tests/m0018-type-checking-core-proposal.sh && sh docs/tests/m0018-type-checking-ambiguity-blocker.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0018-type-checking-core-accepted.sh`
  - `docs/tests/m0018-type-checking-core-concrete-draft.sh`
  - `docs/tests/m0018-type-checking-core-review.sh`
  - `docs/tests/m0018-type-checking-core-proposal.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/SPEC.md`
  - `docs/ambiguities/M0018-type-checking-core.md`
  - `docs/adr/proposals/reviews/ADR-0027-chief-architect-decision.md`
  - `docs/tasks/M0018-005-accept-type-checking-core-adr.md`
  - `docs/tasks/reviews/M0018-005-review.md`
  - `docs/tasks/soundness/M0018-005-soundness.md`

## Forbidden Changes

- Do not implement type checking.
- Do not add well-typed or ill-typed fixtures.
- Do not add primitive type records.
- Do not activate direct calls or function type application.
- Do not weaken or delete existing M0018 validators to hide missing accepted semantics.

## Ambiguities And Dependencies

- M0018 implementation remains dependent on this task until accepted ADR-0027 and SPEC updates are present.
- Direct calls, structural function type application, overloads, numeric conversions, member lookup, generic solving, ownership, borrowing, HIR, MIR, and backend behavior remain deferred after this task.

## Execution Log

- 2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created ADR-0027 acceptance task.
- 2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0018-type-checking-core-accepted.sh before accepting ADR-0027.
- 2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=Accepted-state validator failed before implementation because docs/adr/ADR-0027-type-checking-core.md was missing.
- 2026-07-10 main_task=Chief-Architect phase=ordinary-tests result=pass notes=Accepted ADR-0027, updated SPEC, resolved M0018 ambiguity, approved main task decision, and aligned M0018 validators for accepted state.
- 2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0018-005-accept-type-checking-core-adr.md` created a passing soundness report.
- 2026-07-10 main_task=main-task review phase=review result=pass notes=`docs/scripts/review-task.sh docs/tasks/M0018-005-accept-type-checking-core-adr.md` created review and concrete review approved acceptance-only scope.
- 2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, `sh docs/tests/m0018-type-checking-core-accepted.sh`, `sh docs/tests/m0018-type-checking-core-concrete-draft.sh`, `sh docs/tests/m0018-type-checking-core-review.sh`, and `sh docs/tests/m0002-workspace-ci.sh` passed.
