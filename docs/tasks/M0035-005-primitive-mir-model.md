# Task: M0035-005 Primitive MIR Model

## Task Metadata

- Task ID: `M0035-005`
- Milestone: `M0035`
- Milestone File: `docs/milestones/M0035-primitive-runtime-support.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Extend MIR with typed Bool, Float, and Byte constants plus an explicit Unit
no-result instruction and return terminator.

## Authority Extract

- ADR-0059 defines `i8`, `f64`, unsigned Byte, and Unit no-result ABI behavior.
- ADR-0045 requires backend-independent typed values, source mapping, and
  explicit terminators.
- Existing MIR Int constants and return values remain compatible.

## Scope

- Add Bool, Float-bit-pattern, and Byte constant instructions.
- Add Unit no-result instruction and Unit return terminator.
- Preserve instruction/terminator source spans and exact payloads.
- Add focused MIR model tests.

## Out Of Scope

- HIR-to-MIR primitive lowering.
- Primitive operations, traps, ownership cleanup, or control flow changes.
- Cranelift instruction selection or ABI emission.

## Tests

- Each non-Unit constant preserves payload and source span.
- Unit return has no value operand.
- Existing Int MIR tests remain unchanged.

## Acceptance Criteria

- MIR does not reinterpret non-Int constants as Int.
- Unit returns are representable without a fabricated MirValueId.
- All new instructions remain backend-independent and source-mapped.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=existing
  MIR has only IntConstant and value-bearing Return, while ADR-0059 requires
  typed non-Int values and Unit no-result boundaries. handoff=test-first
- 2026-07-11 main_task=main phase=test-first result=fail evidence=primitive
  constant constructors and ReturnUnit did not exist. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=MIR now
  preserves Bool, Float-bit-pattern, Byte, and Unit forms with source spans.
  handoff=ordinary-tests
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=the
  focused primitive MIR model test passed. handoff=adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=Unit
  has no value operand and non-Int payloads remain distinct. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0045,
  ADR-0059, SPEC.md, M0035, and task scope are aligned. handoff=ci

## Required Outputs

- Authority read: ADR-0045, ADR-0059, M0035, MIR source, and existing tests.
- Files changed: this task, MIR model, and focused tests.
- Tests written before implementation and expected pre-implementation failure:
  primitive MIR constructor and Unit-return tests must fail because the model
  variants do not exist.
- Validation commands and results: primitive planning, identity, frontend,
  typing, HIR, and MIR validators, formatter, Clippy with warnings denied,
  workspace tests, and diff checks passed.
- Open questions or `none`.
- Remaining risk and the next main-task action.
- Remaining risk: HIR-to-MIR lowering and Cranelift primitive lowering remain
  subsequent M0035 tasks. Next main-task action: commit locally.
