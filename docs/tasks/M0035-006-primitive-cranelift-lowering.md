# Task: M0035-006 Primitive Cranelift Lowering

## Task Metadata

- Task ID: `M0035-006`
- Milestone: `M0035`
- Milestone File: `docs/milestones/M0035-primitive-runtime-support.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Lower Bool, Byte, Float, and Unit MIR values to Cranelift using ADR-0059
representations and ABI rules.

## Authority Extract

- ADR-0059 defines Bool/Byte `i8`, Float `f64`, and Unit no-result lowering.
- ADR-0046 defines bootstrap target-aware calling convention boundaries.
- Existing Cranelift lowering and Int arithmetic behavior remain authoritative.

## Scope

- Lower Bool and Byte constants to `i8`.
- Lower Float bit patterns to `f64` constants.
- Lower Unit constants/returns without ABI result values.
- Permit supported primitive return types in Cranelift verification.
- Add focused Cranelift IR tests.

## Out Of Scope

- Primitive arithmetic and comparisons.
- HIR-to-MIR primitive lowering.
- Function parameters, conversions, runtime traps, or public ABI.
- Changes to Int lowering.

## Tests

- Bool, Byte, and Float constant-return functions verify in Cranelift IR.
- Unit function verifies with a zero-result signature and return terminator.
- Unsupported runtime types remain rejected.
- Existing backend suite remains passing.

## Acceptance Criteria

- Cranelift IR uses `i8` for Bool and Byte, `f64` for Float, and no Unit
  result.
- Bool constants are normalized to 0/1.
- Float bit patterns are preserved exactly.
- No non-Int value is reinterpreted as an Int.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0035 HIR
  and MIR now preserve primitive values, while backend lowering still accepts
  only Int return types and Int constants. handoff=test-first
- 2026-07-11 main_task=main phase=test-first result=fail evidence=primitive
  Cranelift lowering initially rejected non-Int return types and constants.
  handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=Bool and
  Byte use i8, Float uses f64, and Unit uses a zero-result signature and return.
  handoff=ordinary-tests
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=the
  focused Cranelift primitive test passed. handoff=adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=Unit
  produces no result and unsupported runtime types remain rejected. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0046,
  ADR-0059, SPEC.md, M0035, and task scope are aligned. handoff=ci

## Required Outputs

- Authority read: ADR-0046, ADR-0059, M0035, backend source, and existing tests.
- Files changed: this task, Cranelift lowering, and focused backend tests.
- Tests written before implementation and expected pre-implementation failure:
  primitive Cranelift tests must fail because only Int ABI/lowering exists.
- Validation commands and results: primitive planning, identity, frontend,
  typing, HIR, MIR, and Cranelift validators, formatter, Clippy with warnings
  denied, workspace tests, and diff checks passed.
- Open questions or `none`.
- Remaining risk and the next main-task action.
- Remaining risk: HIR-to-MIR primitive lowering and primitive operations remain
  subsequent M0035 tasks. Next main-task action: commit locally.
