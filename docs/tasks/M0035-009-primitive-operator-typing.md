# Task: M0035-009 Primitive Operator Typing

## Task Metadata

- Task ID: `M0035-009`
- Milestone: `M0035`
- Milestone File: `docs/milestones/M0035-primitive-runtime-support.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Type the ADR-0059 Bool, Float, and Byte operator families without weakening
the existing Int operator checker or introducing numeric conversion.

## Authority Extract

- ADR-0059 defines Bool logical/equality, Float arithmetic/comparisons, and
  Byte exact-type arithmetic/bitwise/shift operations.
- Existing M0028 Int operator typing remains authoritative for Int.

## Scope

- Add unary Bool `!` typing.
- Add Bool logical/equality, Float arithmetic/comparison, and Byte exact-type
  operator typing.
- Preserve left/right operand order and exact result types.
- Add focused positive and negative type-check tests.

## Out Of Scope

- HIR/MIR operator representation or lowering.
- Runtime traps, float constant folding, or short-circuit control-flow lowering.
- Numeric conversions, promotions, or changes to Int semantics.

## Tests

- Accepted Bool, Float, and Byte operator expressions receive expected types.
- Mixed primitive operands are rejected.
- Bool logical operators return Bool; Float arithmetic returns Float;
  comparisons return Bool; Byte arithmetic/bitwise/shift returns Byte.

## Acceptance Criteria

- Only the operator families defined by ADR-0059 are accepted.
- No implicit conversion occurs between Int, Byte, or Float.
- Existing M0028 operator tests remain passing.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=primitive
  values now reach typed HIR/MIR constants, but operator typing remains Int-only.
  handoff=test-first
- 2026-07-11 main_task=main phase=test-first result=fail evidence=the primitive
  operator checker did not exist and adding unary Not exposed the Int constant
  evaluator's non-exhaustive match. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=Bool,
  Float, and Byte operator families now have exact result typing. handoff=ordinary-tests
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=the
  focused primitive operator test passed. handoff=adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=mixed
  primitive operands do not gain an implicit conversion path and Unit is not
  accepted as an operator fallback. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0059,
  SPEC.md, M0035, and task scope are aligned. handoff=ci

## Required Outputs

- Authority read: ADR-0059, M0028 type checker, parser operators, M0035, and
  existing tests.
- Files changed: this task, parser unary operator metadata, type-check operator
  logic, and focused tests.
- Tests written before implementation and expected pre-implementation failure:
  primitive operator tests must fail because no primitive operator checker exists.
- Validation commands and results recorded in this task's execution log.
- Open questions or `none`.
- Remaining risk and the next main-task action.
- Remaining risk: HIR/MIR operation variants and backend operation lowering
  remain subsequent M0035 tasks. Next main-task action: commit locally.
