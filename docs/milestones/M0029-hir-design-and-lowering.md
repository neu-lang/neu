# M0029: HIR Design And Lowering

## Title

M0029: HIR Design And Lowering

## Identifier

M0029

## Goal

Design and implement lowering from checked AST into HIR.

## Motivation

HIR separates source-oriented frontend structure from later backend-independent lowering while preserving diagnostics links.

## Background

The expected compiler architecture includes AST, semantic checks, HIR, MIR, and backend stages.

## Prerequisites

- M0028

## Inputs

- Checked AST from M0018-M0028.
- Executable HIR contract from ADR-0044.
- Source span model from M0005.
- Diagnostic contract from M0004.

## Outputs

- HIR representation.
- AST-to-HIR lowering.
- HIR validation tests.

## Scope

- Backend-independent HIR for approved checked constructs.
- Preservation of source mapping.

## Out of Scope

- MIR.
- Optimization.
- Code generation.

## Deliverables

- HIR model.
- Lowering pass.
- HIR snapshots or structural tests.

## Acceptance Criteria

- Approved checked fixtures lower to HIR.
- Executable-subset fixtures preserve ADR-0044 runtime facts.
- HIR preserves source mapping sufficient for diagnostics.
- Ill-formed unchecked input is not accepted as a HIR source.

## Test Strategy

- Positive lowering tests.
- Structural HIR snapshot tests.
- Regression tests for source mapping.

## Risks

- HIR may duplicate AST if boundaries are unclear.
- Lowering may accidentally erase semantic information needed by MIR.

## Estimated Effort

3-5 working days.

## Expected Files Changed

- HIR files.
- Lowering files.
- Tests.

## Completion Checklist

- [x] HIR model exists.
- [x] Checked AST lowers to HIR.
- [x] Source mapping is preserved.
