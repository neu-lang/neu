# M0020: Generic Constraints And Capability Bounds

## Title

M0020: Generic Constraints And Capability Bounds

## Identifier

M0020

## Goal

Implement generic parameter and capability-bound representation for approved
generic syntax.

## Motivation

Ownership, copyability, send/share, and nullability will need generic
constraints, but ADR-0032 defers enforcement until those semantic inputs exist.

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

- Generic parameter type representation.
- Capability-bound representation.
- Explicit enforcement deferral.

## Scope

- Accepted generic parameter and bound syntax metadata.
- Parameter type records and opaque capability-bound records.

## Out of Scope

- Higher-kinded types.
- Template metaprogramming.
- Full monomorphization strategy.
- Generic constraint solving, capability semantics, generic argument checking,
  and bound-violation diagnostics.

## Deliverables

- Generic parameter and bound representation fixtures.
- Explicit deferral coverage.

## Acceptance Criteria

- Accepted generic parameter and bound syntax is represented in source order.
- Parameter types and bound occurrences preserve exact AST identity.
- Constraint enforcement is explicitly deferred by ADR-0032.

## Test Strategy

- Generic declaration metadata tests.
- Parameter type and bound record tests.
- Deferred-enforcement boundary tests.

## Risks

- Capability semantics remain intentionally deferred to a post-M0024 decision.

## Estimated Effort

4-5 working days.

## Expected Files Changed

- Parser/type representation files.
- Tests and explicit deferral records.

## Completion Checklist

- [x] Generic parameter and bound records are represented.
- [x] Enforcement deferral is recorded.
- [x] Unsupported generic features are rejected or blocked.
