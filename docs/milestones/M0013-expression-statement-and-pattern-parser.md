# M0013: Expression, Statement, And Pattern Parser

## Title

M0013: Expression, Statement, And Pattern Parser

## Identifier

M0013

## Goal

Implement parsing for approved expression, statement, and pattern syntax.

## Motivation

The frontend needs executable bodies and pattern forms before name resolution and type checking can be meaningfully tested.

## Background

ADR-0011 and ADR-0012 require smart casts and pattern matching, but exact grammar must come from the grammar authority ledger.

## Prerequisites

- M0012

## Inputs

- Grammar authority ledger from M0008.
- AST model from M0009.
- Parser recovery architecture from M0010.
- Relevant ADRs.

## Outputs

- Expression parser for approved expression forms.
- Statement parser for approved statement forms.
- Pattern parser for approved pattern forms.

## Scope

- Approved expression, statement, and pattern syntax.
- Parser diagnostics for these forms.

## Out of Scope

- Type checking.
- Exhaustiveness checking.
- Flow typing.
- Borrow or ownership analysis.

## Deliverables

- Parser implementation for approved body syntax.
- Positive and negative fixtures.
- Diagnostic snapshots.

## Acceptance Criteria

- Approved body fixtures parse to AST.
- Invalid body syntax reports source-spanned diagnostics.
- Parser does not accept constructs marked ambiguous by M0008.

## Test Strategy

- Positive parser fixtures.
- Negative parser fixtures.
- Parser recovery fixtures.
- Diagnostic snapshots.

## Risks

- Operator precedence may be unspecified.
- Pattern grammar may be incomplete.

## Estimated Effort

4-5 working days.

## Expected Files Changed

- Parser files.
- Fixtures.
- Diagnostics snapshots.
- Ambiguity reports if needed.

## Completion Checklist

- [x] Expression fixtures pass.
- [x] Statement fixtures pass.
- [x] Pattern fixtures pass.
- [x] Expression, statement, and pattern AST shell exists.
- [x] Parser implementation for approved body syntax exists.
- [x] Invalid body syntax reports source-spanned diagnostics.
- [x] Deferred body constructs remain rejected.
- [x] Ambiguous syntax remains blocked.
