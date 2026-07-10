# Task: <TASK_ID> <Title>

## Task Metadata

- Task ID: `<TASK_ID>`
- Milestone: `<MILESTONE_ID>`
- Milestone File: `docs/milestones/<MILESTONE_FILE>.md`
- Status: `draft`
- Owner Agent: `Test Engineer | Implementer | Diagnostics Engineer | Build Engineer | other`
- Created By: `Task Decomposer`
- Created Date: `YYYY-MM-DD`
- Branch: `task/<TASK_ID>-<slug>`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/<ADR>.md`
- Project Rules: `AGENTS.md`
- Agent Prompts:
  - `.codex/agents/task-decomposer.toml`
  - `.codex/agents/test-engineer.toml`
  - `.codex/agents/implementer.toml`
  - `.codex/agents/reviewer.toml`

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
- Reviewer approval required to modify/delete failing tests: `yes`

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
- [ ] Reviewer compares output against `docs/SPEC.md` and the milestone.
- [ ] CI passes as final gate.
- [ ] Milestone checklist is updated.

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
- Do not weaken or delete failing tests without reviewer approval.
- Do not implement work outside this task scope.
- Do not introduce language semantics not present in `docs/SPEC.md` or `docs/adr/`.

## Ambiguities And Dependencies

- <Record missing spec or milestone decisions here. If any item blocks safe implementation, file `AMBIGUITY_REPORT_TEMPLATE.md` output and stop.>

## Execution Log

Append entries as the task progresses.

```text
YYYY-MM-DD agent=<agent> phase=<phase> result=<result> notes=<notes>
```

## Handoff

- Next Agent: `<agent>`
- Reason: `<why this agent receives the task next>`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - Relevant ADRs
  - Milestone file
