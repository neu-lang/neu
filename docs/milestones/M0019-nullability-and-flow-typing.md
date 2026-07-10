# M0019: Nullability And Flow Typing

## Title

M0019: Nullability And Flow Typing

## Identifier

M0019

## Goal

Implement nullability checks and flow-sensitive smart casts for approved forms.

## Motivation

Kotlin-like ergonomics require nullability and smart casts, but they must cooperate with borrowing and mutation rules later.

## Background

ADR-0006 defines nullable types. ADR-0011 permits flow-sensitive smart casts for immutable or exclusively borrowed values, invalidated by mutation.

## Prerequisites

- M0018

## Inputs

- Type checker core from M0018.
- `docs/adr/ADR-0006-nullability-and-absence.md`
- `docs/adr/ADR-0011-flow-typing-and-smart-casts.md`
- Mutability decision from ADR-0013.

## Outputs

- Nullability checking.
- Flow refinement tracking.
- Diagnostics for invalid nullable use and invalidated refinement.

## Scope

- Approved nullable operations.
- Smart casts for approved immutable and exclusive cases.

## Out of Scope

- Full borrow checker.
- Pattern exhaustiveness.
- Advanced control-flow constructs not yet specified.

## Deliverables

- Flow typing pass or type checker extension.
- Positive and negative nullability fixtures.
- Diagnostic snapshots.

## Acceptance Criteria

- Non-nullable values cannot implicitly contain null.
- Nullable misuse produces diagnostics.
- Mutation invalidates refinements in tested specified cases.

## Test Strategy

- Positive smart-cast fixtures.
- Negative nullability fixtures.
- Negative refinement invalidation fixtures.
- Diagnostic snapshots.

## Risks

- Exact null-test syntax may be unspecified.
- Control-flow graph requirements may exceed current parser subset.

## Estimated Effort

4-5 working days.

## Expected Files Changed

- Type or flow analysis files.
- Tests.
- Diagnostic snapshots.
- Ambiguity reports if needed.

## Completion Checklist

- [x] Nullable checks pass.
- [x] Smart casts are tested.
- [x] Mutation invalidation is tested.
