# M0026: Unsafe And FFI Boundary Analysis

## Title

M0026: Unsafe And FFI Boundary Analysis

## Identifier

M0026

## Goal

Implement semantic checks for explicit unsafe and FFI boundary declarations approved by the spec.

## Motivation

Systems programming requires unsafe and FFI, but safe-code guarantees must not be weakened.

## Background

ADR-0018 selects explicit unsafe functions and blocks with module-level audit boundaries and safe wrappers for ordinary use.

## Prerequisites

- M0025

## Inputs

- Type checker and safety analyses from M0018-M0025.
- `docs/adr/ADR-0018-unsafe-ffi-and-trust-boundaries.md`
- `docs/adr/ADR-0020-portability-targets-and-platform-semantics.md`

## Outputs

- Unsafe boundary checking.
- FFI declaration validation for approved metadata.
- Diagnostics for unsafe use outside approved contexts.

## Scope

- Explicit unsafe blocks and functions where syntax is specified.
- FFI metadata checks where specified.

## Out of Scope

- Foreign binding generation.
- Full ABI lowering.
- Platform-specific linker behavior.

## Deliverables

- Unsafe/FFI semantic checks.
- Positive unsafe-boundary fixtures.
- Negative unsafe misuse fixtures.
- Diagnostic snapshots.

## Acceptance Criteria

- Unsafe operations outside explicit unsafe context are rejected.
- FFI declarations lacking required specified safety metadata are rejected.
- Diagnostics distinguish proven safety from trusted assertions.

## Test Strategy

- Positive unsafe wrapper fixtures.
- Negative unsafe misuse fixtures.
- FFI metadata tests.
- Diagnostic snapshots.

## Risks

- Unsafe syntax and FFI metadata may be under-specified.
- ABI details are deferred to target-pack milestones.

## Estimated Effort

3-5 working days.

## Expected Files Changed

- Unsafe/FFI analysis files.
- Tests.
- Diagnostic snapshots.
- Ambiguity reports.

## Completion Checklist

- [ ] Unsafe context checks exist.
- [ ] FFI metadata checks exist where specified.
- [ ] Safe-code guarantee remains intact.

