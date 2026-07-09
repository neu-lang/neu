# Task: M0004-001 Create Diagnostic Infrastructure Contract

## Task Metadata

- Task ID: `M0004-001`
- Milestone: `M0004`
- Milestone File: `docs/milestones/M0004-diagnostic-infrastructure-contract.md`
- Status: `complete`
- Owner Agent: `Diagnostics Engineer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-09`
- Branch: `task/M0004-001-diagnostic-contract`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- Project Rules: `docs/AGENTS.md`
- Agent Prompts:
  - `.codex/agents/task-decomposer.md`
  - `.codex/agents/test-engineer.md`
  - `.codex/agents/diagnostics-engineer.md`
  - `.codex/agents/reviewer.md`

## Goal

Define the compiler-wide diagnostic data contract and an inert diagnostic snapshot shape before any compiler stage emits real errors.

## Motivation

M0004 requires diagnostic structure before lexer, parser, type, ownership, borrow, async, or concurrency diagnostics exist.

## Scope

- Document diagnostic severity, primary span, secondary spans, notes, and optional safe suggestions.
- Document that user-facing diagnostics must avoid internal compiler jargon.
- Add an inert diagnostic snapshot fixture proving the snapshot shape.
- Add a validation script for the diagnostic contract and snapshot shape.

## Out Of Scope

- Specific parser, type checker, ownership, borrow, lifetime, async, or thread-safety diagnostics.
- Error recovery implementation.
- Compiler diagnostic rendering code.
- Language accept/reject behavior.

## Required Inputs

- Milestone: `docs/milestones/M0004-diagnostic-infrastructure-contract.md`
- Spec sections:
  - M0004 milestone acceptance criteria.
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- ADRs:
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- Existing files:
  - `docs/test-harness.md`
  - `.codex/agents/diagnostics-engineer.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - `docs/tests/m0004-diagnostic-contract.sh` verifies diagnostic contract documentation and inert snapshot fields.
- Negative tests:
  - The validation script must fail before implementation because `docs/diagnostics.md` and the inert snapshot are absent.
- Diagnostic tests:
  - The inert snapshot must represent diagnostic shape without testing a compiler diagnostic.
- Adversarial tests:
  - Confirm the inert snapshot does not encode parser/type/ownership/borrow/lifetime/concurrency behavior.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0004-diagnostic-contract.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Diagnostic contract documentation and inert snapshot fixture do not exist yet.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add `docs/diagnostics.md` and one inert diagnostic snapshot metadata file that documents required fields without asserting any real compiler error.

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
- [x] Milestone checklist is updated.

## Execution Commands

- Generate tests: `create docs/tests/m0004-diagnostic-contract.sh`
- Verify tests fail: `docs/tests/m0004-diagnostic-contract.sh`
- Ordinary tests: `docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`
- Adversarial tests: `docs/tests/m0004-diagnostic-contract.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0004-001-diagnostic-contract.md`
- CI: `docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0004-diagnostic-contract.sh`
- Implementation files:
  - `docs/diagnostics.md`
  - `tests/golden/diagnostics/M0004-inert.diagnostic.toml`
- Documentation or checklist files:
  - `docs/milestones/M0004-diagnostic-infrastructure-contract.md`
  - `docs/tasks/M0004-001-diagnostic-contract.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not weaken or delete failing tests without reviewer approval.
- Do not implement work outside this task scope.
- Do not introduce language semantics not present in `docs/SPEC.md` or `docs/adr/`.

## Ambiguities And Dependencies

- No semantic ambiguity blocks this task because the snapshot is inert and defines shape only.
- Later milestones must add diagnostic content only when a compiler stage has source-of-truth behavior.

## Execution Log

Append entries as the task progresses.

```text
2026-07-09 agent=Task-Decomposer phase=create-task result=pass notes=Created first M0004 task and narrowed it to diagnostic contract documentation plus inert snapshot shape.
2026-07-09 agent=Test-Engineer phase=generate-tests result=pass notes=Created docs/tests/m0004-diagnostic-contract.sh before implementation.
2026-07-09 agent=Test-Engineer phase=verify-tests-fail result=pass notes=Validation failed as expected: missing docs/diagnostics.md.
2026-07-09 agent=Diagnostics-Engineer phase=ordinary-tests result=pass notes=docs/tests/m0004-diagnostic-contract.sh, m0003 fixture validation, and m0002 workspace validation passed.
2026-07-09 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=Soundness report approved; inert diagnostic snapshot encodes shape only.
2026-07-09 agent=Reviewer phase=review result=pass notes=Review approved against docs/SPEC.md and M0004.
2026-07-09 agent=Diagnostics-Engineer phase=ci result=pass notes=Final CI-equivalent gate m0004, m0003, and m0002 validations passed.
```

## Handoff

- Next Agent: `Roadmap Planner`
- Reason: `M0004 is complete; select M0005 next.`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/milestones/M0004-diagnostic-infrastructure-contract.md`
