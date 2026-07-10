# Task: M0018-002 Draft Type Checking Core Proposal

## Task Metadata

- Task ID: `M0018-002`
- Milestone: `M0018`
- Milestone File: `docs/milestones/M0018-type-checking-core.md`
- Status: `complete`
- Owner Agent: `Language Designer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0018-002-type-checking-core-proposal`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/ADR-0026-name-resolution-policy.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`
- Ambiguity: `docs/ambiguities/M0018-type-checking-core.md`

## Goal

Draft a non-authoritative ADR proposal for the M0018 bootstrap type-checking core rules.

## Motivation

M0018 is blocked for well-typed and ill-typed fixtures because literal typing, primitive scalar categories, assignment compatibility, call resolution, and function type application are not accepted semantics.

## Scope

- Add `docs/adr/proposals/ADR-0027-type-checking-core.md`.
- Explain competing designs and trade-offs.
- Recommend a draft bootstrap subset.
- List required accepted content before implementation.
- Keep `docs/ambiguities/M0018-type-checking-core.md` open.
- Add a validator for proposal completeness.

## Out Of Scope

- Accepting ADR-0027.
- Updating `docs/SPEC.md`.
- Updating accepted ADRs under `docs/adr/`.
- Implementing type checking.
- Adding positive or negative type-check fixtures that depend on unresolved rules.
- Resolving the M0018 ambiguity.

## Required Inputs

- Milestone: `docs/milestones/M0018-type-checking-core.md`
- Ambiguity report: `docs/ambiguities/M0018-type-checking-core.md`
- Current M0017 type model.
- Accepted ADRs listed in Source Of Truth.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Proposal file exists with non-authority notice, competing designs, recommended draft choice, required accepted content, diagnostics, deferrals, consequences, and dependencies.
- Negative tests:
  - Accepted ADR-0027 remains absent.
  - `docs/SPEC.md` is not updated with ADR-0027.
  - Type-check implementation remains diagnostic-only.
- Diagnostic tests:
  - Proposal names required future diagnostics for type mismatch, unresolved type rule, unsupported type rule, and ambiguous type rule.
- Adversarial tests:
  - Proposal must not infer behavior from current parser, current tests, or external language behavior as authority.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0018-type-checking-core-proposal.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `docs/adr/proposals/ADR-0027-type-checking-core.md` does not exist.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a draft proposal only. Do not promote it into accepted ADRs, update SPEC, or implement type checking.

## Acceptance Criteria

- [ ] Task references exactly one milestone.
- [ ] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing change.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] Reviewer compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [x] M0018 remains blocked pending accepted source-of-truth authority.

## Execution Commands

- Generate tests: `create docs/tests/m0018-type-checking-core-proposal.sh`
- Verify tests fail: `sh docs/tests/m0018-type-checking-core-proposal.sh`
- Ordinary tests: `sh docs/tests/m0018-type-checking-core-proposal.sh && sh docs/tests/m0018-type-checking-ambiguity-blocker.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0018-002-type-checking-core-proposal.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0018-002-type-checking-core-proposal.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0018-type-checking-core-proposal.sh && sh docs/tests/m0018-type-checking-ambiguity-blocker.sh && sh docs/tests/m0017-unsupported-type-form-blocking.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0018-type-checking-core-proposal.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/adr/proposals/ADR-0027-type-checking-core.md`
  - `docs/tasks/M0018-002-type-checking-core-proposal.md`
  - `docs/tasks/reviews/M0018-002-review.md`
  - `docs/tasks/soundness/M0018-002-soundness.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted ADRs under `docs/adr/`.
- Do not close `docs/ambiguities/M0018-type-checking-core.md`.
- Do not implement type checking.
- Do not implement literal typing, primitive scalar categories, assignment compatibility, call resolution, or function type application.

## Ambiguities And Dependencies

- M0018 remains blocked by `docs/ambiguities/M0018-type-checking-core.md`.
- Required follow-up reviews: Language Lawyer, Diagnostics Engineer, Adversarial Engineer, Spec Compliance Auditor, Simplicity Guardian, Chief Architect.

## Execution Log

- 2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created M0018 type checking core proposal task.
- 2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Created docs/tests/m0018-type-checking-core-proposal.sh before adding ADR-0027 proposal.
- 2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=Proposal validator failed before implementation because docs/adr/proposals/ADR-0027-type-checking-core.md was missing.
- 2026-07-10 agent=Language-Designer phase=ordinary-tests result=pass notes=Added non-authoritative ADR-0027 type checking core proposal; M0018 proposal and ambiguity blocker validators passed.
- 2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0018-002-type-checking-core-proposal.md` created a passing soundness report.
- 2026-07-10 agent=Reviewer phase=review result=pass notes=`docs/scripts/review-task.sh docs/tasks/M0018-002-type-checking-core-proposal.md` created review and concrete review approved proposal-only scope.
- 2026-07-10 agent=Build-Engineer phase=ci result=pass notes=`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, `sh docs/tests/m0018-type-checking-core-proposal.sh`, `sh docs/tests/m0018-type-checking-ambiguity-blocker.sh`, `sh docs/tests/m0017-unsupported-type-form-blocking.sh`, and `sh docs/tests/m0002-workspace-ci.sh` passed.
