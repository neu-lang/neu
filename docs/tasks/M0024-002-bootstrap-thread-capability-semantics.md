# Task: M0024-002 Bootstrap Thread Capability Semantics

## Task Metadata

- Task ID: `M0024-002`
- Milestone: `M0024`
- Milestone File: `docs/milestones/M0024-thread-safety-capability-analysis.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner main task: `main-task semantic design`

## Objective

Resolve the M0024 thread-capability semantics blocker with an accepted
source-of-truth decision.

## Authority Extract

- `docs/SPEC.md`, “ADR-0014: Thread Safety And Data-Race Freedom”.
- `docs/adr/ADR-0014-thread-safety-and-data-race-freedom.md`.
- `docs/adr/ADR-0032-generic-constraint-enforcement-sequencing.md`.
- `docs/adr/ADR-0035-bootstrap-ownership-and-move-analysis.md`.
- `docs/adr/ADR-0036-bootstrap-borrow-and-lifetime-analysis.md`.
- `docs/ambiguities/M0024-thread-capability-semantics.md`.
- `docs/milestones/M0024-thread-safety-capability-analysis.md`.

## Scope

- Define the bootstrap capability names.
- Define conservative type-category capability derivation.
- Define metadata-only boundary and capture records.
- Define missing-capability diagnostics.
- Keep source-level concurrency syntax and synchronization APIs deferred.

## Out Of Scope

- Compiler implementation.
- Parser support for task spawning, async blocks, or coroutine syntax.
- Generic capability-bound enforcement.
- User-declared capability implementations.
- Runtime scheduling or synchronization primitives.

## Required Tests

- `docs/tests/m0024-thread-capability-semantics-accepted.sh` verifies the
  accepted ADR, SPEC summary, ambiguity resolution, and task evidence.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Accepted ADR defines `Send` and `Share`.
- [x] Accepted ADR defines type-category derivation.
- [x] Accepted ADR defines boundary and capture records.
- [x] Accepted ADR defines `missing_thread_capability`.
- [x] Ambiguity report is resolved.
- [x] No compiler implementation is added.

## Execution Log

- 2026-07-11 agent=Main phase=semantic-resolution result=pass evidence=ADR-0037 accepted; SPEC and M0024 ambiguity report updated. handoff=Task-Decomposer
