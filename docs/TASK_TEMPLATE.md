# Task: <TASK_ID> <Title>

## Task Metadata

- Task ID: `<TASK_ID>`
- Milestone: `<MILESTONE_ID>`
- Milestone File: `docs/milestones/<MILESTONE_FILE>.md`
- Status: `draft`
- Owner main task: `main-task test work | main-task implementation | main-task diagnostics check | main-task build check | other`
- Created By: `main-task task planning`
- Created Date: `YYYY-MM-DD`
- Branch: `task/<TASK_ID>-<slug>`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/<ADR>.md`
- Project Rules: `main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

<One precise engineering outcome.>

## Motivation

<Why this task is needed for the milestone.>

## Scope

- <Included work item 1>
- <Included work item 2>

## Out Of Scope

- <Explicitly excluded work item 1>
- <Explicitly excluded work item 2>

## Required Inputs

- Milestone: `docs/milestones/<MILESTONE_FILE>.md`
- Spec sections:
  - `<section>`
- ADRs:
  - `<ADR>`
- Existing files:
  - `<path>`

## Authority Extract

This is the bounded context for the next main task. List exact sections, ADR
headings, paths, and commands. Do not require whole documents unless the task
is a semantic or architectural decision.

- Required source-of-truth excerpts:
  - `<SPEC heading or ADR heading>`
- Required implementation/test paths:
  - `<path>`
- Validation commands:
  - `<command>`
- Context expansion trigger:
  - `<what missing fact requires escalation or broader reading>`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - <test expectation>
- Negative tests:
  - <test expectation>
- Diagnostic tests:
  - <test expectation or `not applicable`>
- Adversarial tests:
  - <soundness or misuse case>

## Test-First Gate

- Test files to create before implementation:
  - `<path>`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - <The missing behavior this task will implement.>
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

<Smallest implementation change expected to satisfy the tests. Do not include source code or pseudocode in task files.>

## Acceptance Criteria

- [ ] Task references exactly one milestone.
- [ ] Scope and out-of-scope are explicit.
- [ ] Tests are created before implementation.
- [ ] Tests fail before implementation for the expected reason.
- [ ] Implementation is the smallest passing change.
- [ ] Ordinary tests pass.
- [ ] Adversarial tests pass after ordinary tests.
- [ ] main-task review compares output against `docs/SPEC.md` and the milestone.
- [ ] CI passes as final gate.
- [ ] Milestone checklist is updated.

## Review Routing

- Base review: `main-task review`
- main-task test work: `<required only when tests changed | not required>`
- main-task specification check: `<required only when semantic accept/reject behavior changes | not required>`
- main-task diagnostics check: `<required only when diagnostics change | not required>`
- main-task build check: `<required only when build/release files change | not required>`
- main-task simplicity check: `<required only when a new abstraction or boundary is introduced | not required>`
- main-task adversarial check: `<required only when a soundness boundary changes | not required>`

## Execution Commands

Commands may be `blocked: <reason>` until the project has the relevant harness.

- Generate tests: `<command or blocked: reason>`
- Verify tests fail: `<command or blocked: reason>`
- Ordinary tests: `<command or blocked: reason>`
- Adversarial tests: `<command or blocked: reason>`
- Review: `docs/scripts/review-task.sh <task-file>`
- CI: `<command or blocked: reason>`

## Files Expected To Change

- Test files:
  - `<path>`
- Implementation files:
  - `<path>`
- Documentation or checklist files:
  - `<path>`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not weaken or delete failing tests without main-task review approval.
- Do not implement work outside this task scope.
- Do not introduce language semantics not present in `docs/SPEC.md` or `docs/adr/`.

## Ambiguities And Dependencies

- <Record missing spec or milestone decisions here. If any item blocks safe implementation, file `AMBIGUITY_REPORT_TEMPLATE.md` output and stop.>

## Execution Log

Append one concise entry per phase. Do not repeat the task, prior entries, or
full command output.

```text
YYYY-MM-DD main_task=<main task> phase=<phase> result=<pass|fail|blocked> evidence=<command or finding> handoff=<next role|none>
```

## Handoff

- Next main task: `<main task>`
- Reason: `<why this main task receives the task next>`
- Required Context:
  - This task's Authority Extract
  - `<only additional path required for the handoff>`
