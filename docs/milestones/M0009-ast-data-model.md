# M0009: AST Data Model

## Title

M0009: AST Data Model

## Identifier

M0009

## Goal

Define the AST data model for the approved syntax subset.

## Motivation

Parser implementation needs a stable AST target before individual grammar areas are implemented.

## Background

The AST must preserve source spans for diagnostics and avoid encoding semantic analysis decisions prematurely.

## Prerequisites

- M0008

## Inputs

- Grammar authority ledger from M0008.
- Source span model from M0005.
- Diagnostic contract from M0004.

## Outputs

- AST node model for approved syntax.
- AST span preservation rules.

## Scope

- AST structure for approved syntax categories.
- No name resolution or type semantics.

## Out of Scope

- HIR.
- MIR.
- Parser implementation beyond AST construction tests.

## Deliverables

- AST module or model documentation.
- AST construction tests using approved fixture shapes.

## Acceptance Criteria

- AST nodes preserve source spans.
- AST can represent approved module, declaration, type, expression, statement, and pattern categories as far as the grammar ledger allows.
- Ambiguous syntax remains unrepresented or explicitly deferred.

## Test Strategy

- Unit tests for AST construction and span retention.
- Snapshot tests for AST debug output if approved.

## Risks

- AST may accidentally include semantic concepts.
- Incomplete grammar may require deferred nodes.

## Estimated Effort

3-5 working days.

## Expected Files Changed

- AST files.
- AST tests.
- AST documentation.

## Completion Checklist

- [x] AST model exists.
- [x] Span retention is tested.
- [x] Semantic analysis is not encoded in AST.
