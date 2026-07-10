# M0028: Executable Expression Frontend Completion

## Title

M0028: Executable Expression Frontend Completion

## Identifier

M0028

## Goal

Complete parser, type-checker, and diagnostic support for the executable
operator and function subset defined by ADR-0040 through ADR-0043.

## Motivation

HIR and backend milestones must consume accepted executable semantics rather
than inventing missing frontend behavior. The expanded arithmetic subset
includes operators that were not all present in the earlier bootstrap parser
grammar.

## Background

ADR-0042 defines the first runnable source subset. ADR-0043 defines integer
runtime behavior for arithmetic, exponentiation, bitwise, and shift operators.

## Prerequisites

- M0027

## Inputs

- ADR-0040 through ADR-0043.
- Parser grammar from ADR-0024.
- Type checking from M0018.
- Safety analyses through M0026.

## Outputs

- Executable-subset parser coverage.
- Executable-subset type checking.
- Entry-point, call, return, and arithmetic diagnostics.
- Negative tests for unsupported executable forms.

## Scope

- Source forms needed by the first runnable executable subset.
- Integer operator typing for unary, binary arithmetic, exponentiation, shifts,
  and bitwise operations.
- Entry-point and explicit return checking.
- Direct same-module top-level function call checking.

## Out of Scope

- HIR.
- MIR.
- Backend lowering.
- Object emission or linking.
- Runtime shim implementation.
- Standard library.
- Heap allocation.
- Printing.
- Coroutines, unsafe, and FFI runtime behavior.

## Deliverables

- Parser updates for executable operators where missing.
- Type-checker updates for executable operators.
- Entry-point and return analysis.
- Direct call analysis.
- Diagnostic tests and fixtures.

## Acceptance Criteria

- The executable arithmetic and bitwise operator set from ADR-0042 parses.
- The executable arithmetic and bitwise operator set from ADR-0043 type-checks
  for `Int`.
- Missing, duplicate, and invalid `main` forms diagnose.
- Direct function calls and explicit returns follow ADR-0041.
- Unsupported executable forms fail before HIR.
- No HIR, MIR, backend, object, or linker implementation is added.

## Test Strategy

- Positive parser and type-checker fixtures for executable operators.
- Negative fixtures for overflow, division/modulo by zero, negative exponent,
  invalid shifts, invalid calls, invalid entry points, and unsupported
  executable forms.
- Diagnostic snapshot tests for new executable diagnostics.

## Risks

- Operator precedence extensions must not conflict with existing ADR-0024
  precedence without an explicit superseding decision.
- Executable-subset checking may duplicate type-checker logic if boundaries are
  unclear.

## Estimated Effort

3-5 working days.

## Expected Files Changed

- Parser files.
- Type-checker files.
- Diagnostic files.
- Tests.

## Completion Checklist

- [x] Executable operators parse.
- [x] Executable operators type-check.
- [ ] Entry, call, and return checks exist.
- [ ] Unsupported executable forms are rejected before HIR.
