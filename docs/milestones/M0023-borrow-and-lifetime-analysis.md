# M0023: Borrow And Lifetime Analysis

## Title

M0023: Borrow And Lifetime Analysis

## Identifier

M0023

## Goal

Implement borrow and lifetime analysis for approved synchronous code.

## Motivation

Compile-time memory safety requires enforcing shared immutable or exclusive mutable borrowing and preventing invalid lifetimes.

## Background

ADR-0002 defines borrowing. ADR-0003 defines inferred lifetimes with explicit annotations only where needed.

## Prerequisites

- M0022

## Inputs

- Ownership analysis from M0022.
- `docs/adr/ADR-0002-borrowing-semantics.md`
- `docs/adr/ADR-0003-lifetime-model.md`
- Diagnostic contract.

## Outputs

- Borrow analysis for approved synchronous constructs.
- Lifetime validity checks.
- Borrow conflict diagnostics.

## Scope

- Shared immutable borrows.
- Exclusive mutable borrows.
- Local lifetime inference for approved cases.

## Out of Scope

- Async suspension borrowing.
- Advanced explicit lifetime syntax if unspecified.
- Unsafe escape hatches.

## Deliverables

- Borrow checker pass for synchronous subset.
- Positive borrow fixtures.
- Negative aliasing and lifetime fixtures.
- Diagnostic snapshots.

## Acceptance Criteria

- Multiple shared immutable borrows are accepted where specified.
- Simultaneous mutable and shared conflicts are rejected.
- Borrow diagnostics identify conflicting borrow sites.

## Test Strategy

- Positive borrow fixtures.
- Negative aliasing fixtures.
- Negative lifetime escape fixtures.
- Diagnostic snapshots.

## Risks

- Explicit lifetime annotation syntax may be unspecified.
- Borrow splitting rules for collections are deferred.

## Estimated Effort

4-5 working days.

## Expected Files Changed

- Borrow analysis files.
- Tests.
- Diagnostic snapshots.
- Ambiguity reports.

## Completion Checklist

- [ ] Borrow conflicts are diagnosed.
- [ ] Lifetime escape is diagnosed.
- [ ] Async borrowing remains out of scope.

