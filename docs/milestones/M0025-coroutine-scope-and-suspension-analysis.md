# M0025: Coroutine Scope And Suspension Analysis

## Title

M0025: Coroutine Scope And Suspension Analysis

## Identifier

M0025

## Goal

Implement structured coroutine scope and suspension safety analysis for approved async forms.

## Motivation

Structured concurrency must interact soundly with ownership, borrowing, cancellation, and lifetimes.

## Background

ADR-0008 selects structured concurrency. ADR-0009 restricts borrows across suspension unless proven safe.

## Prerequisites

- M0024

## Inputs

- Borrow checker from M0023.
- Thread capability analysis from M0024.
- `docs/adr/ADR-0008-structured-concurrency-semantics.md`
- `docs/adr/ADR-0009-async-suspension-and-borrowing.md`

## Outputs

- Structured task scope analysis.
- Suspension borrow checks.
- Async cancellation resource-safety checks where specified.

## Scope

- Approved coroutine and task-scope forms.
- Borrow-across-suspension rejection or acceptance where provable.

## Out of Scope

- Runtime scheduler.
- Detached task implementation beyond semantic checks.
- Advanced pinned frame annotations if syntax is unspecified.

## Deliverables

- Coroutine semantic analysis.
- Positive structured concurrency fixtures.
- Negative suspension-borrow fixtures.
- Diagnostic snapshots.

## Acceptance Criteria

- Child task lifetime violations are rejected for approved forms.
- Unsafe borrow across suspension is rejected.
- Diagnostics identify the suspension point and borrowed value.

## Test Strategy

- Positive structured-scope fixtures.
- Negative task escape fixtures.
- Negative borrow-across-suspension fixtures.
- Diagnostic snapshots.

## Risks

- Coroutine syntax may be unspecified.
- Cancellation semantics may require further ADR detail.

## Estimated Effort

4-5 working days.

## Expected Files Changed

- Coroutine analysis files.
- Tests.
- Diagnostic snapshots.
- Ambiguity reports.

## Completion Checklist

- [x] Structured scope checks exist.
- [x] Suspension borrow checks exist.
- [x] Runtime scheduling remains out of scope.
