# M0027: Executable Semantics Planning

## Title

M0027: Executable Semantics Planning

## Identifier

M0027

## Goal

Define the source-of-truth executable semantics required before HIR, MIR,
Cranelift, object emission, linking, and executable smoke tests.

## Motivation

Backend milestones must not invent executable behavior. A minimal runnable
program needs accepted entry-point, call, return, arithmetic, IR, ABI, and
runtime/linking contracts before implementation proceeds.

## Background

M0027 records accepted ADRs for a small source-backed executable subset,
including a runnable program that computes an integer result and returns it as
the process exit code.

## Prerequisites

- M0026

## Inputs

- `docs/adr/ADR-0020-portability-targets-and-platform-semantics.md`
- `docs/adr/ADR-0022-declaration-syntax.md`
- `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- `docs/adr/ADR-0026-name-resolution-policy.md`
- `docs/adr/ADR-0027-type-checking-core.md`
- `docs/adr/ADR-0035-bootstrap-ownership-and-move-analysis.md`
- Accepted ADRs ADR-0040 through ADR-0047.

## Outputs

- Program entry-point semantics.
- Function call and return semantics.
- Minimal executable source subset.
- Integer arithmetic runtime semantics.
- HIR runtime contract.
- MIR runtime contract.
- Bootstrap ABI and calling convention contract.
- Object/link/minimal runtime model.

## Scope

- Planning and source-of-truth updates only.
- Roadmap and milestone revisions for M0028 through M0034.
- Documentation validators for the executable-semantics plan.

## Out of Scope

- Compiler implementation.
- HIR implementation.
- MIR implementation.
- Backend implementation.
- Object emission or linking implementation.
- Runtime shim implementation.
- Example updates for runnable programs before implementation exists.

## Deliverables

- ADR-0040 through ADR-0047.
- `docs/SPEC.md` executable-semantics sections.
- Updated `docs/ROADMAP.md`.
- Updated milestone files M0028 through M0034.
- Documentation validators.

## Acceptance Criteria

- Program entry point is specified.
- Function call and return semantics are specified.
- Minimal executable subset is specified and includes integer literals,
  arithmetic and bitwise operations, locals, function declarations, returns,
  direct calls, and `main`.
- Integer runtime semantics specify representation, overflow, division, and
  modulo behavior.
- HIR and MIR runtime contracts are specified.
- ABI/calling convention and object/link/runtime contracts are specified.
- M0031 and M0032 acceptance criteria depend on these contracts.
- M0034 remains release hardening and does not invent executable semantics.

## Test Strategy

- Documentation validator for ADR presence and key semantic decisions.
- Roadmap validator for milestone ordering and renumbering.

## Risks

- The first executable subset may still be too broad for a single backend smoke
  milestone.
- Host-target object/link assumptions may need a later toolchain-specific ADR.

## Estimated Effort

1-2 working days.

## Expected Files Changed

- ADR files.
- `docs/SPEC.md`.
- `docs/ROADMAP.md`.
- Milestone files.
- Documentation test scripts.

## Completion Checklist

- [x] Executable semantics ADRs exist.
- [x] Roadmap is revised before HIR.
- [x] Backend and linker milestones depend on executable semantics.
