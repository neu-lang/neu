# M0028: MIR Design And Lowering

## Title

M0028: MIR Design And Lowering

## Identifier

M0028

## Goal

Design and implement lowering from HIR into backend-independent MIR.

## Motivation

MIR gives the compiler a stable representation for future optimization and backend emission without tying semantics to Cranelift.

## Background

The expected architecture places MIR before optimization and backend code generation.

## Prerequisites

- M0027

## Inputs

- HIR from M0027.
- Type and safety information from M0018-M0026.
- `docs/adr/ADR-0019-compile-time-evaluation-and-metaprogramming.md`

## Outputs

- MIR representation.
- HIR-to-MIR lowering.
- MIR validation tests.

## Scope

- Backend-independent MIR for approved constructs.
- Explicit control-flow representation.

## Out of Scope

- Optimization.
- Cranelift lowering.
- ABI finalization.

## Deliverables

- MIR model.
- Lowering pass.
- MIR structural tests.

## Acceptance Criteria

- Approved HIR fixtures lower to MIR.
- MIR represents control flow without source-language ambiguity.
- MIR remains independent of Cranelift-specific APIs.

## Test Strategy

- HIR-to-MIR lowering tests.
- MIR structural snapshot tests.
- Source mapping regression tests.

## Risks

- MIR may prematurely encode backend constraints.
- Destructor and cancellation lowering may need further semantic detail.

## Estimated Effort

3-5 working days.

## Expected Files Changed

- MIR files.
- Lowering files.
- Tests.
- Ambiguity reports if needed.

## Completion Checklist

- [ ] MIR model exists.
- [ ] HIR lowers to MIR.
- [ ] MIR is backend-independent.

