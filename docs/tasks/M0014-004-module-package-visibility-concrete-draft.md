# Task: M0014-004 Revise module package and visibility proposal into concrete draft

## Task Metadata

- Task ID: `M0014-004`
- Milestone: `M0014`
- Milestone File: `docs/milestones/M0014-module-package-and-visibility-model.md`
- Status: `complete`
- Owner main task: `main-task semantic design`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0014-004-module-package-visibility-concrete-draft`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/proposals/ADR-0025-module-package-visibility-model.md`
  - `docs/adr/proposals/reviews/ADR-0025-language-lawyer-review.md`
  - `docs/adr/proposals/reviews/ADR-0025-build-engineer-review.md`
  - `docs/adr/proposals/reviews/ADR-0025-spec-compliance-review.md`
  - `docs/adr/proposals/reviews/ADR-0025-simplicity-review.md`
  - `docs/adr/proposals/reviews/ADR-0025-chief-architect-decision.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Revise the non-authoritative ADR-0025 proposal with concrete draft module identity, package mapping, visibility metadata, and diagnostic obligations sufficient for later acceptance review.

## Motivation

The ADR-0025 reviews identify concrete blockers: private visibility, default visibility, bootstrap module identity input, source-file assignment, package absence, minimal metadata, and diagnostics. These must be resolved in the draft before main task can accept the model.

## Scope

- Update `docs/adr/proposals/ADR-0025-module-package-visibility-model.md` with concrete draft rules.
- Resolve review blockers in draft text.
- Keep ADR-0025 non-authoritative.
- Keep `docs/ambiguities/M0014-module-package-visibility-model.md` open.
- Add a concrete-draft validator.

## Out Of Scope

- Accepting ADR-0025.
- Updating `docs/SPEC.md`.
- Updating accepted ADRs under `docs/adr/`.
- Implementing module metadata.
- Implementing visibility extraction.
- Resolving the M0014 ambiguity.

## Required Inputs

- Proposal: `docs/adr/proposals/ADR-0025-module-package-visibility-model.md`
- Reviews: `docs/adr/proposals/reviews/ADR-0025-*.md`
- Ambiguity: `docs/ambiguities/M0014-module-package-visibility-model.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Proposal contains concrete draft model sections for module identity, package mapping, visibility metadata, diagnostics, and deferrals.
- Negative tests:
  - Accepted ADR-0025 remains absent.
  - `docs/SPEC.md` remains unchanged for ADR-0025.
  - Module implementation remains absent.
- Diagnostic tests:
  - Concrete draft diagnostic table includes names, primary span or external input location, recovery action, and safe suggestion policy.
- Adversarial tests:
  - Concrete draft keeps package manager, manifests, module dependencies, target-pack artifacts, name resolution, and sealed scope deferred.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0014-module-package-visibility-model-concrete-draft.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - ADR-0025 proposal does not yet contain concrete draft model sections and diagnostic table.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Revise the proposal only. Do not move the draft into accepted ADRs or modify compiler behavior.

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
- [x] M0014 remains blocked pending accepted model authority.

## Execution Commands

- Generate tests: `create docs/tests/m0014-module-package-visibility-model-concrete-draft.sh`
- Verify tests fail: `docs/tests/m0014-module-package-visibility-model-concrete-draft.sh`
- Ordinary tests: `docs/tests/m0014-module-package-visibility-model-concrete-draft.sh && docs/tests/m0014-module-package-visibility-model-review.sh && docs/tests/m0014-module-package-visibility-model-proposal.sh`
- Adversarial tests: `docs/tests/m0014-module-package-visibility-model-concrete-draft.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0014-004-module-package-visibility-concrete-draft.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0014-module-package-visibility-model-concrete-draft.sh && docs/tests/m0014-module-package-visibility-model-review.sh && docs/tests/m0014-module-package-visibility-model-proposal.sh && docs/tests/m0014-module-package-visibility-model-blocked.sh && docs/tests/m0013-expression-statement-pattern-parser-implementation.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0014-module-package-visibility-model-concrete-draft.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/adr/proposals/ADR-0025-module-package-visibility-model.md`
  - `docs/tasks/M0014-004-module-package-visibility-concrete-draft.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted ADRs under `docs/adr/`.
- Do not close `docs/ambiguities/M0014-module-package-visibility-model.md`.
- Do not implement module model code.
- Do not treat the proposal as source of truth.

## Ambiguities And Dependencies

- M0014 remains blocked until ADR-0025 is accepted.

## Execution Log

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0014 concrete draft revision task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Created concrete draft validator before revising ADR-0025 proposal.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=docs/tests/m0014-module-package-visibility-model-concrete-draft.sh failed before implementation because concrete draft model sections were missing.
2026-07-10 main_task=Language-Designer phase=implementation result=pass notes=Revised ADR-0025 proposal with concrete draft module identity, package namespace, visibility metadata, module metadata, and diagnostics.
2026-07-10 main_task=Language-Designer phase=ordinary-tests result=pass notes=M0014 concrete draft, review, proposal, and blocker validators passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0014-004-soundness.md after ordinary-test evidence.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0014-004-review.md approves concrete draft scope.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=Focused M0014 concrete draft validation command passed.
```

## Handoff

- Next main task: `main task`
- Reason: `Review revised ADR-0025 for possible acceptance.`
- Required Context:
  - This task file
  - `docs/adr/proposals/ADR-0025-module-package-visibility-model.md`
  - `docs/adr/proposals/reviews/ADR-0025-*.md`
