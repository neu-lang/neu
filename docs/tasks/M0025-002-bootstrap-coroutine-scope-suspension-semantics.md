# Task: M0025-002 Bootstrap Coroutine Scope Suspension Semantics

## Task Metadata

- Task ID: `M0025-002`
- Milestone: `M0025`
- Milestone File: `docs/milestones/M0025-coroutine-scope-and-suspension-analysis.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner main task: `main-task semantic design`

## Objective

Resolve the M0025 semantic blocker with an accepted metadata-only bootstrap
model for structured task scope and suspension-borrow analysis.

## Authority Extract

- `docs/adr/ADR-0008-structured-concurrency-semantics.md`.
- `docs/adr/ADR-0009-async-suspension-and-borrowing.md`.
- `docs/adr/ADR-0036-bootstrap-borrow-and-lifetime-analysis.md`.
- `docs/adr/ADR-0037-bootstrap-thread-capability-analysis.md`.
- `docs/adr/ADR-0038-bootstrap-coroutine-scope-and-suspension-analysis.md`.
- `docs/SPEC.md`, `ADR-0038: Bootstrap Coroutine Scope And Suspension
  Analysis`.
- `docs/ambiguities/M0025-coroutine-scope-suspension-semantics.md`.

## Scope

- Accept a metadata-only source-of-truth decision for M0025.
- Define structured scope and child-task records.
- Define suspension point and suspended-borrow records.
- Define exact bootstrap rejection conditions.
- Define diagnostic identifiers and span obligations.
- Keep unsupported source syntax and runtime scheduling deferred.

## Out Of Scope

- Compiler implementation.
- Parser support for coroutine syntax.
- Runtime scheduler behavior.
- Runtime cancellation propagation.
- Async frame lowering.
- Thread capability rule changes.

## Required Tests

- `docs/tests/m0025-coroutine-scope-suspension-semantics-accepted.sh`
- `docs/tests/m0025-coroutine-scope-suspension-blocked.sh`

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] ADR-0038 is accepted.
- [x] `docs/SPEC.md` records ADR-0038.
- [x] The M0025 ambiguity report is resolved.
- [x] Diagnostic identifiers are defined.
- [x] No compiler implementation is added.
- [x] No examples update is required because no user-written syntax changes.

## Execution Log

- 2026-07-11 agent=Main phase=semantic-resolution result=complete evidence=ADR-0038 accepted metadata-only M0025 semantics and SPEC updated. handoff=main-task implementation planning
