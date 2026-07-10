# Review: <TASK_ID>

## Review Metadata

- Task ID: `<TASK_ID>`
- Milestone: `<MILESTONE_ID>`
- Reviewer: `Reviewer`
- Review Date: `YYYY-MM-DD`
- Decision: `approve | request changes | block pending ambiguity | block pending tests | block pending CI`

## Inputs Read

- Task file: `tasks/<TASK_ID>-<slug>.md`
- Authority Extract: `<copy the exact task sections used>`
- Milestone file: `<only when acceptance criteria or ordering matter>`
- Specification and ADR excerpts: `<exact headings read>`
- Diff or changed files:
  - `<path>`
- Test results:
  - `<path or command output reference>`
- Adversarial report:
  - `<path or not triggered>`

Successful reviews should be no more than 150 words: decision, checks run, and
residual risk. Write a detailed report only for a blocker, ambiguity, or
explicit user request.

## Required Checks

- [ ] Task references exactly one milestone.
- [ ] Scope matches the milestone.
- [ ] Out-of-scope items were not changed.
- [ ] Tests were created before implementation.
- [ ] Pre-implementation test failure was recorded.
- [ ] Implementation did not weaken or delete tests unless reviewer approval is recorded.
- [ ] Ordinary tests passed before adversarial tests.
- [ ] Adversarial check ran after ordinary tests, when triggered by the task.
- [ ] Output was compared against `docs/SPEC.md`.
- [ ] Output was compared against the milestone acceptance criteria.
- [ ] CI passed as the final gate.

## Findings

List blocking findings first, ordered by severity.

```text
Severity: blocker | high | medium | low
File:
Line:
Finding:
Required fix:
Source of truth:
```

## Spec Compliance

- Relevant spec sections:
  - `<section>`
- Relevant ADRs:
  - `<ADR>`
- Compliance decision: `compliant | non-compliant | unsupported by spec | ambiguous`
- Notes:
  - <notes>

## Test Review

- Test-first evidence:
  - <evidence>
- Ordinary test result:
  - <result>
- Adversarial test result:
  - <result>
- CI result:
  - <result>

## Scope Review

- In scope:
  - <observed in-scope change>
- Out of scope:
  - <observed out-of-scope change or `none`>

## Architecture And Maintainability Review

- Maintainability concerns:
  - <concern or `none`>
- Simplicity concerns:
  - <concern or `none`>
- Required specialty reviews:
  - `Spec Compliance Auditor | Diagnostics Engineer | Build Engineer | Adversarial Engineer | Simplicity Guardian | none`

## Decision

<approve, request changes, or block with reason.>

## Follow-Up

- <required follow-up item or `none`>

## Closure

When a narrow fix resolves this review's finding, append one line with the
rechecked evidence and final decision. Do not require unrelated specialty
reviewers to repeat their work.
