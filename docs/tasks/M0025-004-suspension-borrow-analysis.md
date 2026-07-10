# Task: M0025-004 Suspension Borrow Analysis

## Task Metadata

- Task ID: `M0025-004`
- Milestone: `M0025`
- Milestone File: `docs/milestones/M0025-coroutine-scope-and-suspension-analysis.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner main task: `main-task implementer`

## Objective

Implement the ADR-0038 metadata-only suspension-borrow checker for
`borrow_across_suspension` diagnostics.

## Authority Extract

- `docs/SPEC.md`, `ADR-0038: Bootstrap Coroutine Scope And Suspension
  Analysis`.
- `docs/adr/ADR-0038-bootstrap-coroutine-scope-and-suspension-analysis.md`.
- `docs/milestones/M0025-coroutine-scope-and-suspension-analysis.md`.
- `crates/compiler/src/borrow.rs` for `BorrowKind`.

## Scope

- Add suspended-borrow record types.
- Add `borrow_across_suspension` diagnostic representation.
- Accept a suspended borrow only when the suspended frame is not concurrently
  accessible and the frame scope equals the borrowed-value lifetime scope.
- Preserve input order and diagnostic spans.
- Add focused unit tests and docs validator.

## Out Of Scope

- Parser support for coroutine syntax.
- Runtime scheduler behavior.
- Runtime cancellation propagation.
- Async frame lowering.
- Pinned-frame annotations.
- Thread capability rule changes.

## Required Tests

- `crates/compiler/tests/coroutine.rs`
- `docs/tests/m0025-suspension-borrow-analysis.sh`

## Acceptance Criteria

- [x] Tests are added before implementation.
- [x] Non-concurrent same-scope suspended borrows produce no diagnostics.
- [x] Concurrently accessible suspended frames diagnose.
- [x] Suspended frames that outlive borrowed values diagnose.
- [x] Diagnostics identify suspension node, borrow node, borrowed local, and
  rejection reason.
- [x] No scheduler or source coroutine syntax is added.

## Execution Log

- 2026-07-11 agent=Main phase=test-design result=in-progress evidence=suspension-borrow tests prepared before implementation. handoff=main-task implementer
- 2026-07-11 agent=Main phase=implementation result=complete evidence=suspension-borrow analysis implemented and validated. handoff=main-task reviewer
