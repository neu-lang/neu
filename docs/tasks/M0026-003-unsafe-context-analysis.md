# Task: M0026-003 Unsafe Context Analysis

## Task Metadata

- Task ID: `M0026-003`
- Milestone: `M0026`
- Milestone File: `docs/milestones/M0026-unsafe-and-ffi-boundary-analysis.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner main task: `main-task implementer`

## Objective

Implement the ADR-0039 metadata-only unsafe-context checker for
`unsafe_operation_outside_context` diagnostics.

## Authority Extract

- `docs/SPEC.md`, `ADR-0039: Bootstrap Unsafe FFI Boundary Analysis`.
- `docs/adr/ADR-0039-bootstrap-unsafe-ffi-boundary-analysis.md`.
- `docs/milestones/M0026-unsafe-and-ffi-boundary-analysis.md`.

## Scope

- Add safety-basis metadata for `ProvenSafe` and `TrustedUnsafe`.
- Add unsafe context and unsafe operation record types.
- Report trusted unsafe operations whose context is absent or not supplied.
- Preserve input order and diagnostic spans.
- Add focused unit tests and docs validator.

## Out Of Scope

- FFI declaration metadata validation.
- Parser support for unsafe or FFI syntax.
- Target-pack implementation.
- ABI validation.
- Foreign binding generation.
- Safe-wrapper body checking.

## Required Tests

- `crates/compiler/tests/unsafe_boundary.rs`
- `docs/tests/m0026-unsafe-context-analysis.sh`

## Acceptance Criteria

- [x] Tests are added before implementation.
- [x] `ProvenSafe` operations are accepted without unsafe context.
- [x] `TrustedUnsafe` operations are accepted only in supplied unsafe contexts.
- [x] Invalid trusted operations report `unsafe_operation_outside_context`.
- [x] Diagnostics preserve operation span, optional context span, operation
  kind, and safety basis.
- [x] No unsafe source syntax or FFI implementation is added.

## Execution Log

- 2026-07-11 agent=Main phase=test-design result=in-progress evidence=unsafe-context tests prepared before implementation. handoff=main-task implementer
- 2026-07-11 agent=Main phase=implementation result=complete evidence=unsafe context analysis implemented and validated. handoff=main-task reviewer
