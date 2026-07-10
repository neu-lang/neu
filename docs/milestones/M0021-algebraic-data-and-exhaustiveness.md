# M0021: Algebraic Data And Exhaustiveness

## Title

M0021: Algebraic Data And Exhaustiveness

## Identifier

M0021

## Goal

Implement the ADR-0033 bootstrap subset: no-payload enum variants and
exhaustive expression-level `when` checking.

## Motivation

Algebraic data and exhaustive matching are central to safe domain modeling and error handling.

## Background

ADR-0012 selects sealed sums; ADR-0033 defines the first parser-backed finite
coverage subset without payloads or implicit smart casts. ADR-0034 supplies
the narrow enum-typed parameter subject required by that subset.

## Prerequisites

- M0020

## Inputs

- Parser pattern support from M0013.
- Type checker from M0018.
- Flow typing from M0019.
- `docs/adr/ADR-0012-pattern-matching-and-algebraic-data.md`
- `docs/adr/ADR-0034-bootstrap-enum-subject-typing.md`

## Outputs

- Enum variant metadata and identity checks.
- Exhaustiveness checker for qualified no-payload variant arms.
- ADR-0033 diagnostics.

## Scope

- Identifier-only enum variants.
- `when` expressions with qualified variant or wildcard arms.

## Out of Scope

- Open class hierarchies.
- Payloads, destructuring, generic enums, nullable coverage, and implicit smart casts.
- Optimization of match lowering.

## Deliverables

- Enum and match parser metadata.
- Positive exhaustive-match tests.
- Negative duplicate, unknown, and non-exhaustive tests.
- Diagnostic snapshots.

## Acceptance Criteria

- Exhaustive qualified-variant matches pass.
- Non-exhaustive and duplicate/unknown matches diagnose.
- Unsupported payload and destructuring forms remain rejected or blocked.

## Test Strategy

- Positive enum/when fixtures.
- Negative coverage fixtures.
- Diagnostic snapshots.

## Risks

- Payload, destructuring, and nullable coverage are intentionally deferred.

## Estimated Effort

4-5 working days.

## Expected Files Changed

- Type or pattern analysis files.
- Tests.
- Diagnostic snapshots.

## Completion Checklist

- [x] Sealed sums are checked.
- [x] Exhaustiveness is checked.
- [x] Diagnostics identify missing cases.
