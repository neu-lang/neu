# Task: M0035-008 Primitive HIR-To-MIR Lowering

## Task Metadata

- Task ID: `M0035-008`
- Milestone: `M0035`
- Milestone File: `docs/milestones/M0035-primitive-runtime-support.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Lower primitive HIR literals into typed MIR constants and preserve Unit's
no-result return boundary.

## Authority Extract

- ADR-0045 defines MIR typed values and explicit terminators.
- ADR-0059 defines primitive MIR/ABI representations.
- M0035-005 and M0035-007 define the source and model inputs.

## Scope

- Lower Bool, Float, Byte, and Unit HIR literals.
- Accept all four primitive return types in HIR-to-MIR validation.
- Emit `ReturnUnit` for Unit functions.
- Add focused HIR-to-MIR integration tests.

## Out Of Scope

- Primitive arithmetic, comparisons, conversions, parameters, or calls.
- Cranelift changes.
- Byte contextual type transport beyond already typed HIR input.

## Tests

- Bool, Float, and Byte HIR constants lower to their MIR forms.
- Unit HIR literal lowers to a Unit instruction and Unit return terminator.
- Unsupported runtime types remain rejected.

## Acceptance Criteria

- No non-Int HIR literal is reinterpreted as Int.
- Unit lowering never creates a value-bearing return operand.
- Existing Int HIR-to-MIR tests pass.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=HIR now
  lowers requested literal forms, while HIR-to-MIR accepts only Int and always
  emits value-bearing returns. handoff=test-first
- 2026-07-11 main_task=main phase=test-first result=fail evidence=the new
  HIR-to-MIR integration test was rejected by the Int-only runtime guard.
  handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=primitive
  HIR forms lower to typed MIR constants and Unit emits ReturnUnit; the HIR
  literal kind is checked against the owning TypeArena. handoff=ordinary-tests
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=the
  complete MIR suite passed nine tests. handoff=adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=Unit
  has no return operand and foreign TypeArena identities remain rejected.
  handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0045,
  ADR-0059, SPEC.md, M0035, and task scope are aligned. handoff=ci

## Required Outputs

- Authority read: ADR-0045, ADR-0059, M0035-005, M0035-007, MIR, and tests.
- Files changed: this task, HIR-to-MIR lowering, and focused tests.
- Tests written before implementation and expected pre-implementation failure:
  primitive HIR-to-MIR tests must fail because lowering rejects non-Int types.
- Validation commands and results: primitive planning, identity, frontend,
  typing, HIR, MIR, Cranelift, checked-HIR, and HIR-to-MIR validators, formatter,
  Clippy with warnings denied, workspace tests, and diff checks passed.
- Open questions or `none`.
- Remaining risk and the next main-task action.
- Remaining risk: primitive operations and executable smoke coverage remain
  subsequent M0035 tasks. Next main-task action: commit locally.
