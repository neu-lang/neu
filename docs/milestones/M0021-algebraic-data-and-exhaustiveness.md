# M0021: Algebraic Data And Exhaustiveness

## Title

M0021: Algebraic Data And Exhaustiveness

## Identifier

M0021

## Goal

Implement sealed sum type checking and exhaustive pattern matching for approved forms.

## Motivation

Algebraic data and exhaustive matching are central to safe domain modeling and error handling.

## Background

ADR-0012 selects sealed sum types with exhaustive pattern matching integrated with smart casts.

## Prerequisites

- M0020

## Inputs

- Parser pattern support from M0013.
- Type checker from M0018.
- Flow typing from M0019.
- `docs/adr/ADR-0012-pattern-matching-and-algebraic-data.md`

## Outputs

- Sealed sum type semantic checks.
- Exhaustiveness checker.
- Pattern typing diagnostics.

## Scope

- Approved sealed sum declarations.
- Approved exhaustive match forms.

## Out of Scope

- Open class hierarchies.
- Pattern matching syntax not approved by grammar ledger.
- Optimization of match lowering.

## Deliverables

- Exhaustiveness analysis.
- Positive exhaustive-match tests.
- Negative non-exhaustive tests.
- Diagnostic snapshots.

## Acceptance Criteria

- Exhaustive approved matches pass.
- Non-exhaustive approved matches fail with diagnostics.
- Smart-cast interaction is tested for approved cases.

## Test Strategy

- Positive sealed sum fixtures.
- Negative exhaustiveness fixtures.
- Diagnostic snapshots.

## Risks

- Variant evolution rules may be unspecified.
- Pattern grammar may be incomplete.

## Estimated Effort

4-5 working days.

## Expected Files Changed

- Type or pattern analysis files.
- Tests.
- Diagnostic snapshots.

## Completion Checklist

- [ ] Sealed sums are checked.
- [ ] Exhaustiveness is checked.
- [ ] Diagnostics identify missing cases.

