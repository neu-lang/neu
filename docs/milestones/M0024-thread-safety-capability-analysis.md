# M0024: Thread Safety Capability Analysis

## Title

M0024: Thread Safety Capability Analysis

## Identifier

M0024

## Goal

Implement compile-time send/share capability checks for approved concurrent boundary forms.

## Motivation

The language promises compile-time thread safety and data-race freedom.

## Background

ADR-0014 selects compile-time send/share capabilities, derived where sound and explicitly declared where necessary.

## Prerequisites

- M0023

## Inputs

- Type checker from M0018-M0020.
- Borrow checker from M0023.
- `docs/adr/ADR-0014-thread-safety-and-data-race-freedom.md`

## Outputs

- Send/share capability representation.
- Capability checks at approved task or thread boundaries.
- Diagnostics for invalid cross-boundary captures.

## Scope

- Approved capability derivation.
- Approved boundary checks.

## Out of Scope

- Runtime scheduler.
- Actor-only model.
- Lock implementation.

## Deliverables

- Capability analysis pass.
- Positive send/share fixtures.
- Negative data-race-risk fixtures.
- Diagnostic snapshots.

## Acceptance Criteria

- Values lacking required capability cannot cross approved concurrent boundaries.
- Shared mutable state requires approved synchronization abstraction or is rejected.
- Diagnostics name missing capability and capture site.

## Test Strategy

- Positive capability fixtures.
- Negative cross-boundary capture fixtures.
- Diagnostic snapshots.

## Risks

- Exact syntax for task spawning may be unspecified.
- Safe synchronization abstractions may not yet exist.

## Estimated Effort

4-5 working days.

## Expected Files Changed

- Capability analysis files.
- Tests.
- Diagnostic snapshots.
- Ambiguity reports.

## Completion Checklist

- [x] Capability model exists.
- [x] Invalid cross-boundary captures diagnose.
- [ ] Unspecified concurrency forms are blocked.
