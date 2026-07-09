# M0004: Diagnostic Infrastructure Contract

## Title

M0004: Diagnostic Infrastructure Contract

## Identifier

M0004

## Goal

Define the compiler-wide diagnostic data contract before any stage emits real errors.

## Motivation

Diagnostics are a semantic obligation. Ownership, borrowing, lifetimes, nullability, and concurrency errors must remain explainable as the compiler grows.

## Background

ADR-0015 requires diagnostic obligations for all core safety systems.

## Prerequisites

- M0003

## Inputs

- `docs/SPEC.md`
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- `.codex/agents/diagnostics-engineer.md`

## Outputs

- Diagnostic severity model.
- Span and note requirements.
- Snapshot-testing expectations.

## Scope

- Diagnostic contract and minimal infrastructure shape.
- No semantic diagnostic rules.

## Out of Scope

- Specific parser or type checker diagnostics.
- Error recovery implementation.

## Deliverables

- Diagnostic contract documentation.
- Harness support for diagnostic snapshots.

## Acceptance Criteria

- Diagnostic snapshots can represent primary span, secondary spans, message, notes, and optional safe suggestions.
- Contract forbids internal compiler jargon in user-facing messages.
- No language accept/reject behavior is introduced.

## Test Strategy

- Harness-level diagnostic snapshot smoke test.
- Manual review by Diagnostics Engineer.

## Risks

- Underpowered diagnostic structure requiring churn later.
- Overengineering diagnostic categories before real errors exist.

## Estimated Effort

2-3 working days.

## Expected Files Changed

- Diagnostic infrastructure files.
- Diagnostic test harness files.
- Diagnostic documentation.

## Completion Checklist

- [x] Diagnostic shape is documented.
- [x] Snapshot harness supports required fields.
- [x] No specific language rule is encoded.
