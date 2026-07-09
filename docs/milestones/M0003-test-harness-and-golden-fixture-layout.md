# M0003: Test Harness And Golden Fixture Layout

## Title

M0003: Test Harness And Golden Fixture Layout

## Identifier

M0003

## Goal

Define the test harness and fixture layout used by all later compiler stages.

## Motivation

The project requires tests before implementation. Later agents need a consistent way to express positive, negative, and diagnostic expectations.

## Background

ADR-0015 makes diagnostics part of the semantic design. The agent rules require Test Engineer work before Implementer work.

## Prerequisites

- M0002

## Inputs

- `docs/SPEC.md`
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- `AGENTS.md`
- `.codex/agents/test-engineer.md`
- `.codex/agents/diagnostics-engineer.md`

## Outputs

- Test directory structure.
- Golden fixture conventions.
- Negative-test conventions.
- Diagnostic snapshot conventions.

## Scope

- Test infrastructure design and empty harness.
- Fixture naming and metadata rules.

## Out of Scope

- Tests for actual compiler behavior.
- Compiler implementation.

## Deliverables

- Test harness documentation.
- Empty smoke test proving the harness runs.
- Fixture schema or conventions document.

## Acceptance Criteria

- The test command discovers and runs the harness.
- Fixture categories exist for positive, negative, and diagnostic tests.
- Harness documentation states how tests cite spec or ADR authority.

## Test Strategy

- Run the empty harness.
- Add one inert fixture that proves discovery without testing semantics.

## Risks

- Fixture format may overfit future implementation.
- Test metadata may be too weak to enforce spec citation.

## Estimated Effort

2-3 working days.

## Expected Files Changed

- Test harness files.
- Test fixture directories.
- Test documentation.

## Completion Checklist

- [ ] Positive fixture category exists.
- [ ] Negative fixture category exists.
- [ ] Diagnostic snapshot category exists.
- [ ] Tests can cite source-of-truth documents.

