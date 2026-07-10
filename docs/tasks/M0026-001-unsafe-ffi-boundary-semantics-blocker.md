# Task: M0026-001 Unsafe FFI Boundary Semantics Blocker

## Task Metadata

- Task ID: `M0026-001`
- Milestone: `M0026`
- Milestone File: `docs/milestones/M0026-unsafe-and-ffi-boundary-analysis.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner main task: `main-task language review`

## Objective

Record the missing accepted semantics that block M0026 unsafe and FFI boundary
analysis implementation.

## Authority Extract

- `docs/SPEC.md`, `ADR-0018: Unsafe, FFI, And Trust Boundaries`.
- `docs/adr/ADR-0018-unsafe-ffi-and-trust-boundaries.md`.
- `docs/adr/ADR-0020-portability-targets-and-platform-semantics.md`.
- `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`,
  `Unsafe And Coroutine Syntax`.
- `docs/milestones/M0026-unsafe-and-ffi-boundary-analysis.md`.
- `docs/ambiguities/M0026-unsafe-ffi-boundary-semantics.md`.

## Scope

- Create an ambiguity report for missing unsafe and FFI boundary semantics.
- State the minimum resolution required before M0026 implementation.
- Add a validator that keeps implementation from guessing.

## Out Of Scope

- Compiler implementation.
- Parser support for unsafe or FFI syntax.
- Target-pack implementation.
- Source-of-truth resolution.

## Required Tests

- `docs/tests/m0026-unsafe-ffi-boundary-blocked.sh`

## Blocker

M0026 requires approved unsafe-context forms or records, unsafe-operation
records, FFI declaration metadata requirements, target-awareness scope,
safe-wrapper status, and diagnostics. Accepted source of truth currently
selects explicit unsafe and FFI boundaries only at a high level, while
ADR-0024 defers unsafe block syntax.

## Required Resolution

An accepted ADR or spec revision must define either a source-syntax subset or a
metadata-only bootstrap subset for M0026 before implementation can proceed. The
resolution must identify supported records or forms, rejection conditions,
diagnostic identifiers, primary and secondary spans, required FFI metadata, and
whether target-pack validation is included.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Ambiguity report exists.
- [x] Validator confirms M0026 remains blocked.
- [x] No compiler implementation is added.
- [x] No examples update is required because no user-written syntax changes.

## Execution Log

- 2026-07-11 agent=Main phase=blocker-recorded result=blocked evidence=ADR-0018 and ADR-0020 do not define testable M0026 unsafe/FFI records or diagnostics. handoff=main-task semantic design
