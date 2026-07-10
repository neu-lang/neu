# Task: M0025-003 Structured Task Scope Analysis

## Task Metadata

- Task ID: `M0025-003`
- Milestone: `M0025`
- Milestone File: `docs/milestones/M0025-coroutine-scope-and-suspension-analysis.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner main task: `main-task implementer`

## Objective

Implement the ADR-0038 metadata-only structured task scope checker for
`task_scope_escape` diagnostics.

## Authority Extract

- `docs/SPEC.md`, `ADR-0038: Bootstrap Coroutine Scope And Suspension
  Analysis`.
- `docs/adr/ADR-0038-bootstrap-coroutine-scope-and-suspension-analysis.md`.
- `docs/milestones/M0025-coroutine-scope-and-suspension-analysis.md`.

## Scope

- Add structured task scope and child-task record types.
- Add `task_scope_escape` diagnostic representation.
- Analyze ordered scope records and report child tasks whose
  completion-or-cancellation scope differs from the containing scope.
- Preserve input order and diagnostic spans.
- Add focused unit tests and docs validator.

## Out Of Scope

- Suspension-borrow analysis.
- Parser support for coroutine syntax.
- Runtime scheduler behavior.
- Runtime cancellation propagation.
- Thread capability rule changes.
- User-facing examples.

## Required Tests

- `crates/compiler/tests/coroutine.rs`
- `docs/tests/m0025-structured-task-scope-analysis.sh`

## Acceptance Criteria

- [x] Tests are added before implementation.
- [x] Valid child tasks completing in their containing scope produce no diagnostics.
- [x] Escaping child tasks produce `task_scope_escape`.
- [x] Diagnostics preserve task and scope spans.
- [x] Existing unsupported source forms remain unsupported.
- [x] No scheduler or source coroutine syntax is added.

## Execution Log

- 2026-07-11 agent=Main phase=test-design result=in-progress evidence=task and tests prepared before implementation. handoff=main-task implementer
- 2026-07-11 agent=Main phase=implementation result=complete evidence=structured task scope analysis implemented and validated. handoff=main-task reviewer
