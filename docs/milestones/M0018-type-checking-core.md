# M0018: Type Checking Core

## Title

M0018: Type Checking Core

## Identifier

M0018

## Goal

Implement core type checking for the approved typed AST subset.

## Motivation

Safety analyses depend on typed programs. Type checking must precede ownership and borrow checking.

## Background

ADR-0010 establishes nominal types and constrained generics. This milestone implements only spec-backed checking rules.

## Prerequisites

- M0017

## Inputs

- Name resolution from M0016.
- Type representation from M0017.
- Approved parser subset.
- Relevant ADRs.

## Outputs

- Type checking pass.
- Type mismatch diagnostics.

## Scope

- Approved expression and declaration type checking.
- Assignment and call checking only where specified.

## Out of Scope

- Ownership moves.
- Borrow checking.
- Flow typing.
- Generic constraint solving beyond placeholders.

## Deliverables

- Type checker core.
- Positive typed fixtures.
- Negative type mismatch fixtures.
- Diagnostic snapshots.

## Acceptance Criteria

- Approved well-typed fixtures pass type checking.
- Approved ill-typed fixtures fail with source-spanned diagnostics.
- Ambiguous typing rules are recorded and not implemented.

## Test Strategy

- Positive type-check fixtures.
- Negative type-check fixtures.
- Diagnostic snapshots.

## Risks

- Literal typing and overload rules may be unspecified.
- Call syntax and function type rules may require ambiguity reports.

## Estimated Effort

4-5 working days.

## Expected Files Changed

- Type checker files.
- Tests.
- Diagnostic snapshots.

## Completion Checklist

- [x] Well-typed fixtures pass.
- [x] Ill-typed fixtures diagnose.
- [x] Ambiguous type rules are blocked.
