# M0029: Cranelift Backend Smoke

## Title

M0029: Cranelift Backend Smoke

## Identifier

M0029

## Goal

Implement a minimal Cranelift backend path for MIR constructs approved for the first executable smoke test.

## Motivation

The compiler needs an initial backend, but backend work must follow frontend semantics and MIR.

## Background

Project context selects Cranelift initially and LLVM optionally later.

## Prerequisites

- M0028

## Inputs

- MIR from M0028.
- Target assumptions from ADR-0020.
- Build skeleton from M0002.

## Outputs

- Cranelift lowering for a minimal approved MIR subset.
- Backend smoke test artifact.

## Scope

- Minimal Cranelift emission for selected MIR constructs.
- Backend diagnostics for unsupported MIR.

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
- Unsupported MIR constructs fail with clear diagnostics or internal unsupported markers in tests.
- No source semantics are introduced in backend code.

## Test Strategy

- Backend smoke test.
- Unsupported MIR tests.
- Build test for backend crate.

## Risks

- Minimal executable semantics may depend on under-specified entry point rules.
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

