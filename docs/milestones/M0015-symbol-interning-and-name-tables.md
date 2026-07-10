# M0015: Symbol Interning And Name Tables

## Title

M0015: Symbol Interning And Name Tables

## Identifier

M0015

## Goal

Create symbol interning and name table infrastructure independent of resolution policy.

## Motivation

Name resolution needs stable symbol identities and lookup tables before enforcing language-specific visibility and scoping.

## Background

The type system is nominal per ADR-0010, making stable names and symbols central to frontend correctness.

## Prerequisites

- M0014

## Inputs

- Module model from M0014.
- AST from M0009-M0013.
- `docs/adr/ADR-0010-type-system-shape.md`

## Outputs

- Symbol identity infrastructure.
- Name table representation.
- Duplicate-name detection hooks.

## Scope

- Symbol and name infrastructure.
- No final resolution semantics.

## Out of Scope

- Import resolution.
- Type checking.
- Overload resolution unless already specified.

## Deliverables

- Symbol table module.
- Tests for stable symbol identity and table insertion.

## Acceptance Criteria

- Same textual name interns to stable identity within one compilation session.
- Distinct modules can hold distinct symbols with same textual name.
- Duplicate insertion behavior is tested where specified or marked ambiguous.

## Test Strategy

- Unit tests for interning.
- Unit tests for table construction.
- Negative tests only for specified duplicate rules.

## Risks

- Overload and extension-method semantics may be unspecified.
- Premature resolution policy may leak into symbol infrastructure.

## Estimated Effort

2-4 working days.

## Expected Files Changed

- Symbol infrastructure files.
- Tests.

## Completion Checklist

- [x] Symbol identities are stable.
- [ ] Name tables are tested.
- [ ] Resolution policy is deferred.
