# Task: M0026-002 Bootstrap Unsafe FFI Boundary Semantics

## Task Metadata

- Task ID: `M0026-002`
- Milestone: `M0026`
- Milestone File: `docs/milestones/M0026-unsafe-and-ffi-boundary-analysis.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner main task: `main-task semantic design`

## Objective

Resolve the M0026 semantic blocker with an accepted metadata-only bootstrap
model for unsafe operation contexts and FFI safety metadata.

## Authority Extract

- `docs/adr/ADR-0018-unsafe-ffi-and-trust-boundaries.md`.
- `docs/adr/ADR-0020-portability-targets-and-platform-semantics.md`.
- `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`.
- `docs/adr/ADR-0039-bootstrap-unsafe-ffi-boundary-analysis.md`.
- `docs/SPEC.md`, `ADR-0039: Bootstrap Unsafe FFI Boundary Analysis`.
- `docs/ambiguities/M0026-unsafe-ffi-boundary-semantics.md`.

## Scope

- Accept a metadata-only source-of-truth decision for M0026.
- Define safety bases for proven compiler safety and trusted unsafe assertions.
- Define unsafe context and unsafe operation records.
- Define FFI declaration metadata requirements.
- Define diagnostic identifiers and span obligations.
- Keep unsupported source syntax, target packs, and ABI lowering deferred.

## Out Of Scope

- Compiler implementation.
- Parser support for unsafe or FFI syntax.
- Target-pack implementation.
- ABI validation.
- Foreign binding generation.
- Safe-wrapper body checking.

## Required Tests

- `docs/tests/m0026-unsafe-ffi-boundary-blocked.sh`
- `docs/tests/m0026-unsafe-ffi-boundary-semantics-accepted.sh`

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] ADR-0039 is accepted.
- [x] `docs/SPEC.md` records ADR-0039.
- [x] The M0026 ambiguity report is resolved.
- [x] Diagnostic identifiers are defined.
- [x] No compiler implementation is added.
- [x] No examples update is required because no user-written syntax changes.

## Execution Log

- 2026-07-11 agent=Main phase=semantic-resolution result=complete evidence=ADR-0039 accepted metadata-only M0026 semantics and SPEC updated. handoff=main-task implementation planning
