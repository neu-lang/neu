# Task: M0025-001 Coroutine Scope Suspension Semantics Blocker

## Task Metadata

- Task ID: `M0025-001`
- Milestone: `M0025`
- Milestone File: `docs/milestones/M0025-coroutine-scope-and-suspension-analysis.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner main task: `main-task language review`

## Objective

Record the missing accepted semantics that block M0025 coroutine scope and
suspension analysis implementation.

## Authority Extract

- `docs/SPEC.md`, “ADR-0008: Structured Concurrency Semantics” and “ADR-0009:
  Async Suspension And Borrowing”.
- `docs/adr/ADR-0008-structured-concurrency-semantics.md`.
- `docs/adr/ADR-0009-async-suspension-and-borrowing.md`.
- `docs/milestones/M0025-coroutine-scope-and-suspension-analysis.md`.
- `docs/ambiguities/M0025-coroutine-scope-suspension-semantics.md`.

## Scope

- Create an ambiguity report for missing coroutine and suspension semantics.
- State the minimum resolution required before M0025 implementation.
- Add a validator that keeps implementation from guessing.

## Out Of Scope

- Compiler implementation.
- Parser support for coroutine syntax.
- Runtime scheduling.
- Source-of-truth resolution.

## Required Tests

- `docs/tests/m0025-coroutine-scope-suspension-blocked.sh`

## Blocker

M0025 requires approved coroutine/task-scope forms, suspension records, borrow
extension rules, child task escape checks, cancellation resource-safety scope,
and diagnostics. Accepted source of truth currently chooses structured
concurrency and proven-safe suspension borrowing only at a high level.

## Required Resolution

An accepted ADR or spec revision must define either a source-syntax subset or a
metadata-only bootstrap subset for M0025 before implementation can proceed.
The resolution must identify supported records or forms, rejection conditions,
diagnostic identifiers, primary and secondary spans, and whether cancellation
checks are included.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Ambiguity report exists.
- [x] Validator confirms M0025 remains blocked.
- [x] No compiler implementation is added.
- [x] No examples update is required because no user-written syntax changes.

## Execution Log

- 2026-07-11 agent=Main phase=blocker-recorded result=blocked evidence=ADR-0008 and ADR-0009 do not define testable M0025 coroutine forms, suspension records, or diagnostics. handoff=main-task semantic design
