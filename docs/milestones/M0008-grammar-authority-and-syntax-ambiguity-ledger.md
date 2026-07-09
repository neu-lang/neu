# M0008: Grammar Authority And Syntax Ambiguity Ledger

## Title

M0008: Grammar Authority And Syntax Ambiguity Ledger

## Identifier

M0008

## Goal

Create a grammar authority ledger that distinguishes specified syntax from unresolved syntax before parser implementation expands.

## Motivation

Parser work cannot safely proceed by assuming Kotlin grammar. The project needs a durable record of what syntax is authorized.

## Background

`docs/SPEC.md` currently records high-level semantic decisions, not a full grammar.

## Prerequisites

- M0007

## Inputs

- `docs/SPEC.md`
- `docs/adr/`
- Lexer output from M0007.

## Outputs

- Grammar authority ledger.
- Syntax ambiguity reports.
- Parser milestone unblock list.

## Scope

- Planning and ambiguity recording.
- No grammar invention.

## Out of Scope

- Parser implementation.
- Spec modification.
- New language syntax.

## Deliverables

- `docs/syntax/` authority ledger or equivalent.
- Ambiguity reports for missing grammar decisions.

## Acceptance Criteria

- Every planned parser construct is classified as specified, ambiguous, or deferred.
- Ambiguous constructs have owner and blocking milestone listed.
- No parser tests are added for ambiguous constructs.

## Test Strategy

- Manual audit by Language Lawyer and Language Designer.
- Text check that parser fixture metadata references the ledger.

## Risks

- Too many syntax gaps may block parser milestones.
- Ledger could drift from future spec updates.

## Estimated Effort

2-4 working days.

## Expected Files Changed

- Grammar authority documentation.
- Ambiguity reports.

## Completion Checklist

- [x] Syntax constructs are classified.
- [x] Blocking ambiguities are recorded.
- [x] Parser scope is defined.
