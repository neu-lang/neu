# Task: M0003-001 Create Test Harness And Golden Fixture Layout

## Task Metadata

- Task ID: `M0003-001`
- Milestone: `M0003`
- Milestone File: `docs/milestones/M0003-test-harness-and-golden-fixture-layout.md`
- Status: `complete`
- Owner main task: `main-task test work`
- Created By: `main-task task planning`
- Created Date: `2026-07-09`
- Branch: `task/M0003-001-test-harness-fixtures`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Create the initial fixture layout, inert fixture metadata convention, and harness documentation used by later compiler-stage tests.

## Motivation

M0003 requires a repeatable test layout before lexer, parser, diagnostic, or semantic behavior tests are added.

## Scope

- Add positive, negative, and diagnostic fixture categories.
- Add one inert fixture that proves discovery without testing compiler behavior.
- Add harness documentation requiring fixtures to cite `docs/SPEC.md` or ADR authority.
- Add a validation script for the M0003 layout.

## Out Of Scope

- Compiler behavior tests.
- Lexer, parser, AST, semantic, backend, or target-pack implementation.
- Diagnostic rendering implementation.
- Language syntax decisions.

## Required Inputs

- Milestone: `docs/milestones/M0003-test-harness-and-golden-fixture-layout.md`
- Spec sections:
  - M0003 milestone acceptance criteria.
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- ADRs:
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- Existing files:
  - `docs/ROADMAP.md`
  - `docs/main task rules`
  - `main task rules`
  - `main task rules`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - `docs/tests/m0003-fixture-layout.sh` verifies fixture categories, inert fixture metadata, and harness documentation.
- Negative tests:
  - The validation script must fail before implementation because fixture directories and docs are absent.
- Diagnostic tests:
  - The validation script verifies that diagnostic fixture conventions are documented, without testing compiler diagnostics.
- Adversarial tests:
  - Confirm the inert fixture does not encode language syntax or expected compiler behavior.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0003-fixture-layout.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - The fixture layout and harness documentation do not exist yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add the minimum fixture directories, one inert fixture metadata file, and documentation needed for later main tasks to add spec-cited positive, negative, and diagnostic tests.

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
- [x] Milestone checklist is updated.

## Execution Commands

- Generate tests: `create docs/tests/m0003-fixture-layout.sh`
- Verify tests fail: `docs/tests/m0003-fixture-layout.sh`
- Ordinary tests: `docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`
- Adversarial tests: `docs/tests/m0003-fixture-layout.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0003-001-test-harness-fixtures.md`
- CI: `docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0003-fixture-layout.sh`
- Implementation files:
  - `tests/fixtures/positive/M0003-inert.fixture.toml`
  - `tests/fixtures/negative/.gitkeep`
  - `tests/fixtures/diagnostics/.gitkeep`
  - `tests/golden/diagnostics/.gitkeep`
- Documentation or checklist files:
  - `docs/test-harness.md`
  - `docs/milestones/M0003-test-harness-and-golden-fixture-layout.md`
  - `docs/tasks/M0003-001-test-harness-fixtures.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not weaken or delete failing tests without main-task review approval.
- Do not implement work outside this task scope.
- Do not introduce language semantics not present in `docs/SPEC.md` or `docs/adr/`.

## Ambiguities And Dependencies

- No semantic ambiguity blocks this task because fixtures are inert and do not encode compiler behavior.
- Later milestones must add feature-specific fixture schemas only after relevant spec authority exists.

## Execution Log

Append entries as the task progresses.

```text
2026-07-09 main_task=Task-Decomposer phase=create-task result=pass notes=Created first M0003 task and narrowed it to inert fixture layout and documentation.
2026-07-09 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0003-fixture-layout.sh before implementation.
2026-07-09 main_task=main-task test work phase=verify-tests-fail result=pass notes=Validation failed as expected: missing tests/fixtures/positive directory.
2026-07-09 main_task=main-task implementation phase=ordinary-tests result=pass notes=docs/tests/m0003-fixture-layout.sh and docs/tests/m0002-workspace-ci.sh passed after adding inert fixture layout and docs.
2026-07-09 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=Soundness report approved; inert fixture encodes no syntax or compiler expectations.
2026-07-09 main_task=main-task review phase=review result=pass notes=Review approved against docs/SPEC.md and M0003.
2026-07-09 main_task=main-task test work phase=ci result=pass notes=Final CI-equivalent gate docs/tests/m0003-fixture-layout.sh and docs/tests/m0002-workspace-ci.sh passed.
```

## Handoff

- Next main task: `main-task roadmap planning`
- Reason: `M0003 is complete; select M0004 next.`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/milestones/M0003-test-harness-and-golden-fixture-layout.md`
