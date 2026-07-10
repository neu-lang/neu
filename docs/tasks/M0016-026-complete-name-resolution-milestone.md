# Task: M0016-026 Complete name resolution milestone

## Task Metadata

- Task ID: `M0016-026`
- Milestone: `M0016`
- Milestone File: `docs/milestones/M0016-name-resolution-pass.md`
- Status: `complete`
- Owner Agent: `Reviewer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0016-026-complete-name-resolution-milestone`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
- Project Rules: `docs/AGENTS.md`
- Agent Prompts:
  - `.codex/agents/reviewer.md`
  - `.codex/agents/spec-compliance-auditor.md`
  - `.codex/agents/build-engineer.md`

## Goal

Close M0016 by marking its remaining milestone checklist items complete after current implementation and validation prove the acceptance criteria.

## Motivation

The implementation now includes accepted-subset name reference binding, unresolved diagnostics, duplicate diagnostics, and ambiguity deferral. The milestone checklist still has unchecked items for approved names resolving and unresolved names diagnosing. The milestone should only be closed with validator coverage proving those checklist items remain complete.

## Scope

- Extend the M0016 data-model validator to require the completed milestone checklist.
- Update `docs/milestones/M0016-name-resolution-pass.md` checklist items for approved-name resolution and unresolved-name diagnostics.
- Record task, review, and soundness evidence.

## Out Of Scope

- Compiler implementation changes.
- New name-resolution semantics.
- Package-qualified expression lookup.
- Import lookup.
- Cross-module lookup.
- Member, method, constructor, overload, extension, or type-directed lookup.
- Changes to accepted ADR-0026.

## Required Inputs

- `docs/milestones/M0016-name-resolution-pass.md`
- `docs/tests/m0016-name-resolution-data-model.sh`
- `crates/newlang/tests/name_resolution.rs`
- `docs/adr/ADR-0026-name-resolution-policy.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Validator requires the M0016 approved-name resolution checklist item to be checked.
  - Validator requires the M0016 unresolved-name diagnostics checklist item to be checked.
- Negative tests:
  - Validator fails while those items remain unchecked.
- Adversarial tests:
  - Closing the milestone does not add unsupported lookup semantics.

## Test-First Gate

- Test files to update before implementation:
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - M0016 milestone checklist still has unchecked approved-name and unresolved-name items.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add milestone checklist assertions to the M0016 validator, verify they fail, then update only the milestone checklist items supported by current tests and implementation.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing change.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] Reviewer compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.

## Execution Commands

- Generate tests: `update docs/tests/m0016-name-resolution-data-model.sh`
- Verify tests fail: `docs/tests/m0016-name-resolution-data-model.sh`
- Ordinary tests: `docs/tests/m0016-name-resolution-data-model.sh && cargo test -p newlang --test name_resolution && docs/tests/m0016-name-resolution-policy-accepted.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0016-026-complete-name-resolution-milestone.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0016-026-complete-name-resolution-milestone.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh && docs/tests/m0016-name-resolution-blocked.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0016-name-resolution-data-model.sh`
  - `docs/tests/m0016-name-resolution-blocked.sh`
- Documentation files:
  - `docs/milestones/M0016-name-resolution-pass.md`
  - `docs/tasks/M0016-026-complete-name-resolution-milestone.md`

## Forbidden Changes

- Do not modify compiler source.
- Do not add new name-resolution semantics.
- Do not modify accepted ADR-0026.
- Do not weaken or delete existing M0016 tests.

## Ambiguities And Dependencies

- Package-qualified expression lookup remains deferred because parser metadata does not yet distinguish it from member access.
- Unsupported import, cross-module, member, overload, extension, and type-directed lookup remain explicitly outside M0016.

## Execution Log

```text
2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created M0016 milestone completion task.
2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Updated M0016 data-model validator to require completed milestone checklist items.
2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=docs/tests/m0016-name-resolution-data-model.sh failed before milestone update because Approved names resolve remained unchecked.
2026-07-10 agent=Reviewer phase=implementation result=pass notes=Marked M0016 approved-name and unresolved-name checklist items complete based on implemented accepted-subset binders and diagnostics.
2026-07-10 agent=Test-Engineer phase=ordinary-tests result=pass notes=M0016 data-model validator, cargo test -p newlang --test name_resolution, and M0016 accepted-state validator passed.
2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created a soundness report after ordinary tests; concrete adversarial review found milestone closure backed by validators and no unsupported semantic changes.
2026-07-10 agent=Reviewer phase=review result=pass notes=docs/scripts/review-task.sh created a review after adversarial checks; concrete review approved scope pending final CI.
2026-07-10 agent=Build-Engineer phase=ci-fix result=pass notes=Updated legacy M0016 authority validator to require completed checklist items after milestone closure.
2026-07-10 agent=Build-Engineer phase=ci result=pass notes=cargo fmt, cargo clippy, cargo test, M0016 data-model/accepted/authority validators, M0015 validator, and M0002 validator passed.
```

## Handoff

- Next Agent: `Roadmap Planner`
- Reason: `M0016 milestone is complete; select M0017 next.`
- Required Context:
  - This task file
  - `docs/milestones/M0016-name-resolution-pass.md`
  - `docs/tests/m0016-name-resolution-data-model.sh`
