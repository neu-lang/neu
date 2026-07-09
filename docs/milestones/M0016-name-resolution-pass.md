# M0016: Name Resolution Pass

## Title

M0016: Name Resolution Pass

## Identifier

M0016

## Goal

Resolve approved names from AST into symbols using the module and name infrastructure.

## Motivation

Type checking requires names to be bound to declarations before semantic analysis can proceed.

## Background

ADR-0010 requires nominal user-defined types. ADR-0017 defines modules as visibility units.

## Prerequisites

- M0015

## Inputs

- AST from M0009-M0013.
- Module model from M0014.
- Symbol infrastructure from M0015.
- Relevant ADRs.

## Outputs

- Name resolution pass.
- Resolution diagnostics.

## Scope

- Approved local, module, and declaration name resolution.
- Duplicate and unresolved-name diagnostics where specified.

## Out of Scope

- Type inference.
- Method resolution not specified by ADRs.
- Extension-method coherence unless specified.

## Deliverables

- Name resolution implementation.
- Positive and negative resolution tests.
- Diagnostic snapshots.

## Acceptance Criteria

- Approved name references resolve to stable symbols.
- Unresolved specified-name cases produce diagnostics.
- Ambiguous resolution forms are blocked by reports.

## Test Strategy

- Resolution fixture tests.
- Negative unresolved-name tests.
- Diagnostic snapshots.

## Risks

- Import syntax and overload rules may be unspecified.
- Extension-method rules may require future ADR detail.

## Estimated Effort

4-5 working days.

## Expected Files Changed

- Name resolution files.
- Tests.
- Diagnostic snapshots.
- Ambiguity reports.

## Completion Checklist

- [ ] Approved names resolve.
- [ ] Unresolved names diagnose.
- [ ] Ambiguous resolution cases are not guessed.

