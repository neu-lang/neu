# M0006: Token Model And Lexer Fixtures

## Title

M0006: Token Model And Lexer Fixtures

## Identifier

M0006

## Goal

Define the token model and lexer fixtures that later lexing implementation must satisfy.

## Motivation

The lexer must be tested before implementation and must not infer unspecified syntax.

## Background

The language requires Kotlin-like syntax, but detailed lexical grammar is not yet specified in `docs/SPEC.md`.

## Prerequisites

- M0005

## Inputs

- `docs/SPEC.md`
- `docs/adr/`
- Test harness from M0003.
- Source database from M0005.

## Outputs

- Token category plan limited to spec-backed syntax.
- Lexer fixture files.
- Ambiguity report for unspecified lexical details.

## Scope

- Token model planning.
- Test fixtures for known syntax.
- Ambiguity reporting.

## Out of Scope

- Lexer implementation.
- Parser.
- New syntax decisions.

## Deliverables

- Lexer fixture suite.
- Token model document or test metadata.
- Ambiguity report for missing lexical grammar details.

## Acceptance Criteria

- Every lexer fixture cites `docs/SPEC.md` or an ADR.
- Missing lexical rules are recorded as dependencies.
- No fixture encodes guessed syntax.

## Test Strategy

- Fixture metadata validation.
- Manual review by Language Lawyer.

## Risks

- Current spec may be too high-level for meaningful lexer fixtures.
- Kotlin-like syntax may tempt agents to assume Kotlin tokens.

## Estimated Effort

2-4 working days.

## Expected Files Changed

- Lexer test fixtures.
- Lexer test documentation.
- Ambiguity reports.

## Completion Checklist

- [x] Token categories are spec-backed or marked blocked.
- [x] Fixtures cite authority.
- [x] Ambiguities are recorded.
