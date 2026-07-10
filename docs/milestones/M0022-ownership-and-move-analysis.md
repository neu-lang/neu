# M0022: Ownership And Move Analysis

## Title

M0022: Ownership And Move Analysis

## Identifier

M0022

## Goal

Implement ownership and move analysis for approved value categories.

## Motivation

No garbage collector and no manual memory management require ownership to be enforced before borrowing and destruction are complete.

## Background

ADR-0001 selects single-owner affine ownership. ADR-0005 defines primitive copy and user-defined move defaults.

## Prerequisites

- M0021

## Inputs

- Typed program representation from M0018-M0021.
- `docs/adr/ADR-0001-ownership-model.md`
- `docs/adr/ADR-0005-copy-move-and-value-categories.md`

## Outputs

- Move analysis.
- Copyability checks.
- Use-after-move diagnostics.

## Scope

- Approved local ownership flows.
- Primitive copy versus user-defined move behavior.

## Out of Scope

- Borrow checking.
- Destructor execution semantics beyond recording obligations.
- Async frames.

## Deliverables

- Ownership analysis pass.
- Positive move fixtures.
- Negative use-after-move fixtures.
- Diagnostic snapshots.

## Acceptance Criteria

- Copyable primitive scalar fixtures remain usable after copy.
- Moved user-defined values cannot be used afterward.
- Diagnostics identify move origin and invalid use.

## Test Strategy

- Positive copy and move fixtures.
- Negative use-after-move fixtures.
- Diagnostic snapshots.

## Risks

- Exact primitive scalar set may be unspecified.
- Partial moves and destructors may need additional ADR detail.

## Estimated Effort

4-5 working days.

## Expected Files Changed

- Ownership analysis files.
- Tests.
- Diagnostic snapshots.
- Ambiguity reports if needed.

## Completion Checklist

- [x] Move analysis runs after type checking.
- [x] Use-after-move is diagnosed.
- [x] Copyability is tested.
