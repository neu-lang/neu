# M0007: Lexer Implementation

## Title

M0007: Lexer Implementation

## Identifier

M0007

## Goal

Implement lexing for the token subset approved by M0006.

## Motivation

Lexing is the first compiler stage and must provide reliable tokens and diagnostics for later parsing.

## Background

The lexer depends on source spans from M0005 and test fixtures from M0006.

## Prerequisites

- M0006

## Inputs

- Lexer fixtures from M0006.
- Source database from M0005.
- Diagnostic contract from M0004.

## Outputs

- Lexer implementation for approved token subset.
- Lexer diagnostics for approved lexical errors.

## Scope

- Tokenization for spec-backed tokens.
- Lexical error reporting for fixture-backed cases.

## Out of Scope

- Parser behavior.
- Syntax not approved in M0006.
- Semantic interpretation of tokens.

## Deliverables

- Lexer module.
- Passing lexer fixture tests.
- Diagnostic snapshots for lexical errors.

## Acceptance Criteria

- All approved lexer fixtures pass.
- Unknown or ambiguous lexical forms are rejected with diagnostics or marked blocked by fixture metadata.
- Lexer diagnostics include source spans.

## Test Strategy

- Positive tokenization tests.
- Negative lexical error tests.
- Diagnostic snapshot tests.

## Risks

- Incomplete lexical grammar may limit scope.
- Error recovery may be underdefined.

## Estimated Effort

3-5 working days.

## Expected Files Changed

- Lexer module.
- Lexer tests.
- Diagnostic snapshots.

## Completion Checklist

- [ ] Approved lexer fixtures pass.
- [ ] Lexical errors have diagnostics.
- [ ] No guessed syntax is accepted.

