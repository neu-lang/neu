# M0005: Source Database, Spans, And File Identity

## Title

M0005: Source Database, Spans, And File Identity

## Identifier

M0005

## Goal

Establish source-file identity, text storage, and span mapping used by all frontend stages.

## Motivation

Reliable diagnostics and incremental stage testing require stable source locations before lexing, parsing, and semantic analysis.

## Background

ADR-0015 requires actionable diagnostics. Later phases need spans that survive lowering.

## Prerequisites

- M0004

## Inputs

- Diagnostic contract from M0004.
- Test harness from M0003.
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`

## Outputs

- Source file identity model.
- Span representation.
- Line and column mapping behavior.

## Scope

- Source storage and location mapping.
- No language tokenization.

## Out of Scope

- Lexer.
- Parser.
- Module resolution.

## Deliverables

- Source database API contract.
- Tests for file identity and line/column mapping.

## Acceptance Criteria

- Tests prove stable file identifiers for multiple source files.
- Tests prove byte offsets map to line and column positions.
- Diagnostic snapshots can reference mapped spans.

## Test Strategy

- Unit tests for empty files, single-line files, multi-line files, and invalid span boundaries.

## Risks

- Unicode column semantics may be ambiguous.
- Source encoding rules are not yet specified.

## Estimated Effort

2-3 working days.

## Expected Files Changed

- Source database module.
- Span module.
- Tests.

## Completion Checklist

- [x] File identity tests pass.
- [x] Span mapping tests pass.
- [x] Unicode or encoding ambiguity is recorded if unresolved.
