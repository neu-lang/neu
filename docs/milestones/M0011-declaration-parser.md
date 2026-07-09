# M0011: Declaration Parser

## Title

M0011: Declaration Parser

## Identifier

M0011

## Goal

Implement parsing for approved module-level and declaration syntax.

## Motivation

Declarations are required before name resolution, type checking, and module modeling can proceed.

## Background

The parser must target the AST model and grammar authority ledger.

## Prerequisites

- M0010

## Inputs

- Lexer from M0007.
- AST model from M0009.
- Parser recovery architecture from M0010.
- Grammar authority ledger from M0008.

## Outputs

- Declaration parser for approved constructs.
- Declaration parser tests and diagnostics.

## Scope

- Approved module-level declarations.
- Approved visibility syntax only if specified.

## Out of Scope

- Expression parsing.
- Type checking.
- Name resolution.
- Unspecified declaration forms.

## Deliverables

- Declaration parser implementation.
- Positive and negative declaration fixtures.
- Syntax diagnostic snapshots.

## Acceptance Criteria

- Approved declaration fixtures parse to AST.
- Invalid declaration fixtures produce diagnostics with spans.
- Ambiguous declaration syntax remains blocked.

## Test Strategy

- Positive parser fixtures.
- Negative parser fixtures.
- Diagnostic snapshots.

## Risks

- Visibility and module syntax may be under-specified.
- Declaration parser may need temporary AST placeholders.

## Estimated Effort

3-5 working days.

## Expected Files Changed

- Parser files.
- Parser fixtures.
- Diagnostics snapshots.

## Completion Checklist

- [x] Declaration syntax source of truth is accepted.
- [x] Declaration parser fixtures are defined.
- [x] Declaration AST shell exists.
- [ ] Declaration fixtures pass.
- [ ] Invalid declarations produce diagnostics.
- [ ] No type semantics are implemented.
