# Task: M0035-003 Primitive Literal Typing

## Task Metadata

- Task ID: `M0035-003`
- Milestone: `M0035`
- Milestone File: `docs/milestones/M0035-primitive-runtime-support.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Type Float literals as exact `Float` values and support contextual Byte
initialization only for integer literals in `0..255`.

## Authority Extract

- ADR-0059 defines Float exact typing, Byte range, and no implicit conversion.
- Existing initializer checking records exact assignment checks and mismatch
  diagnostics.
- Parser integer metadata already preserves unsigned literal values.

## Scope

- Carry integer literal values into primitive initializer checking.
- Accept in-range integer literal to Byte initialization in Byte context.
- Diagnose out-of-range Byte literals with `byte_literal_out_of_range` rule.
- Preserve Float and Unit literal type identities through initializer checking.

## Out Of Scope

- Numeric casts or Int/Byte arithmetic.
- Float constant folding or non-finite literal diagnostics.
- HIR, MIR, Cranelift, ABI, and runtime traps.

## Tests

- Float literal initializes Float exactly.
- Byte literals `0` and `255` initialize Byte.
- Byte literal `256` is rejected with a stable rule.
- Int-to-Byte and Byte-to-Int implicit conversion remains rejected.

## Acceptance Criteria

- Byte acceptance is limited to literal values in `0..255` and Byte context.
- Out-of-range values do not produce assignment success.
- Float and Unit exact typing remains intact.
- Existing Int initializer behavior and diagnostics remain unchanged.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=ADR-0059
  requires contextual Byte range checking while current initializer typing has
  no integer-value input. handoff=test-first
- 2026-07-11 main_task=main phase=test-first result=fail evidence=the new
  initializer tests failed because the checker accepted no integer-value input
  and had no Byte range rule. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=integer
  literal metadata reaches initializer checking and Byte range validation is
  explicit. handoff=ordinary-tests
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=the
  complete type-check suite passed 100 tests. handoff=adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=256
  is rejected while 0 and 255 are accepted, with no conversion path added.
  handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0059,
  SPEC.md, M0035, and task scope are aligned. handoff=ci

## Required Outputs

- Authority read: ADR-0059, M0035, parser literal metadata, type checker, and
  existing tests.
- Files changed: this task, initializer type checking, and focused tests.
- Tests written before implementation and expected pre-implementation failure:
  the new Byte/Float initializer tests must fail because the checker API and
  Byte range rule do not exist.
- Validation commands and results: formatter, primitive planning, identity,
  literal frontend, and literal typing validators, Clippy with warnings denied,
  workspace tests, and diff checks passed.
- Open questions or `none`.
- Remaining risk and the next main-task action.
- Remaining risk: primitive operations and runtime lowering remain subsequent
  M0035 tasks. Next main-task action: commit locally.
