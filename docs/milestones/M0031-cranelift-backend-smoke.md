# M0031: Cranelift Backend Smoke

## Title

M0031: Cranelift Backend Smoke

## Identifier

M0031

## Goal

Implement a minimal Cranelift backend path for MIR constructs approved for the first executable smoke test.

## Motivation

The compiler needs an initial backend, but backend work must follow frontend semantics and MIR.

## Background

Project context selects Cranelift initially and LLVM optionally later.

## Prerequisites

- M0030

## Inputs

- MIR from M0030.
- Target assumptions from ADR-0020.
- Integer runtime semantics from ADR-0043.
- ABI assumptions from ADR-0046.
- Type-environment transport from ADR-0055.
- Build skeleton from M0002.

## Outputs

- Cranelift lowering for a minimal approved MIR subset.
- Backend smoke test artifact.

## Scope

- Minimal Cranelift emission for selected MIR constructs.
- Backend diagnostics for unsupported MIR.
- Type resolution only through the owning module `TypeArena`.

## Out of Scope

- Optimization.
- LLVM.
- Full ABI coverage.
- Cross-target support beyond smoke target.

## Deliverables

- Cranelift backend module.
- Smoke test from source through backend emission.
- Unsupported-feature diagnostics.

## Acceptance Criteria

- A spec-backed minimal program reaches Cranelift emission.
- Cranelift lowering covers ADR-0043 arithmetic, exponentiation, bitwise, and
  shift operations for the executable subset.
- Unsupported MIR constructs fail with clear diagnostics or internal unsupported markers in tests.
- No source semantics are introduced in backend code.

## Test Strategy

- Backend smoke test.
- Unsupported MIR tests.
- Build test for backend crate.

## Risks

- Minimal executable semantics depend on ADR-0040 through ADR-0047.
- ABI details may require ambiguity reports.

## Estimated Effort

3-5 working days.

## Expected Files Changed

- Backend files.
- Build manifests.
- Backend tests.
- Diagnostic snapshots.

## Completion Checklist

- [ ] Cranelift path exists.
- [ ] Smoke backend test passes.
- [ ] Unsupported constructs are explicit.
