# M0020: Generic Constraints And Capability Bounds

## Title

M0020: Generic Constraints And Capability Bounds

## Identifier

M0020

## Goal

Implement generic constraints and capability-bound representation for approved generic APIs.

## Motivation

Ownership, copyability, send/share, and nullability need generic constraints before safety analyses can be general.

## Background

ADR-0016 selects constrained nominal generics with explicit capability bounds.

## Prerequisites

- M0019

## Inputs

- Type checker from M0018.
- Type representation from M0017.
- `docs/adr/ADR-0016-generics-and-parametric-polymorphism.md`
- `docs/adr/ADR-0014-thread-safety-and-data-race-freedom.md`

## Outputs

- Generic parameter and constraint checking.
- Capability-bound representation.
- Constraint diagnostics.

## Scope

- Approved generic declarations and uses.
- Copyability and send/share bounds as representable constraints where syntax is specified.

## Out of Scope

- Higher-kinded types.
- Template metaprogramming.
- Full monomorphization strategy.

## Deliverables

- Generic constraint checking.
- Positive and negative generic fixtures.
- Diagnostic snapshots.

## Acceptance Criteria

- Approved generic declarations type check.
- Violated specified bounds produce diagnostics.
- Unspecified bound syntax or inference rules are recorded as ambiguities.

## Test Strategy

- Generic declaration tests.
- Generic use tests.
- Negative bound violation tests.
- Diagnostic snapshots.

## Risks

- Capability-bound syntax may be unspecified.
- Static specialization policy may require later backend decisions.

## Estimated Effort

4-5 working days.

## Expected Files Changed

- Type checker files.
- Constraint files.
- Tests.
- Diagnostic snapshots.

## Completion Checklist

- [ ] Generic constraints are represented.
- [ ] Bound violations diagnose.
- [ ] Unsupported generic features are rejected or blocked.

