# M0012: Type And Generic Syntax Parser

## Title

M0012: Type And Generic Syntax Parser

## Identifier

M0012

## Goal

Implement parsing for approved type syntax, nullable type syntax, and generic parameter syntax.

## Motivation

Type checking and capability bounds require parsed type forms before semantic analysis can begin.

## Background

ADR-0006, ADR-0010, and ADR-0016 define high-level type, nullability, and generics choices, but detailed syntax may require ambiguity reports.

## Prerequisites

- M0011

## Inputs

- Grammar authority ledger from M0008.
- AST model from M0009.
- Declaration parser from M0011.
- Relevant ADRs.

## Outputs

- Type syntax parser.
- Generic parameter syntax parser.
- Syntax ambiguity reports for unspecified forms.

## Scope

- Approved type references.
- Approved nullable type syntax.
- Approved generic parameter and bound syntax.

## Out of Scope

- Type checking.
- Constraint solving.
- Capability semantics.

## Deliverables

- Type parser implementation.
- Type syntax fixtures.
- Diagnostic snapshots.

## Acceptance Criteria

- Approved type fixtures parse to AST.
- Invalid type syntax reports source-spanned diagnostics.
- Unspecified generic or capability syntax is blocked by ambiguity reports.

## Test Strategy

- Positive type syntax tests.
- Negative type syntax tests.
- Diagnostic snapshots.

## Risks

- Capability-bound syntax may be unspecified.
- Nullable syntax may be assumed from Kotlin without formal authority.

## Estimated Effort

3-5 working days.

## Expected Files Changed

- Parser files.
- Type syntax fixtures.
- Diagnostic snapshots.
- Ambiguity reports if needed.

## Completion Checklist

- [ ] Approved type syntax parses.
- [ ] Generic syntax scope is explicit.
- [x] Ambiguities are recorded.
