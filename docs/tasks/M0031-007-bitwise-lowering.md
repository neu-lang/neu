# Task: M0031-007 Bitwise Lowering

## Task Metadata

- Task ID: `M0031-007`
- Milestone: `M0031`
- Milestone File: `docs/milestones/M0031-cranelift-backend-smoke.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

## Goal

Lower bootstrap `Int` bitwise AND, OR, and XOR into Cranelift IR.

## Authority Extract

- ADR-0043 signed 64-bit two's-complement bitwise semantics.
- ADR-0045 checked `Int` operations in MIR.
- ADR-0046 bootstrap `Int` ABI lowering.

## Scope

- Lower `BitwiseAnd`, `BitwiseOr`, and `BitwiseXor` in source operand order.

## Out Of Scope

- Exponentiation, shifts, unary operators, calls, locals, object emission,
  linking, and executable startup.

## Test-First Gate

- Test: a chained bitwise MIR function emits `band`, `bor`, and `bxor`.
- Expected initial result: `fail`; all three forms are unsupported.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=ADR-0043
  specifies exact two's-complement behavior with no runtime trap. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=the chained
  bitwise fixture returned `UnsupportedInstruction`. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=all
  three operations lower directly to Cranelift `I64` bitwise instructions.
  handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=focused
  bitwise test and validator passed; formatter, Clippy, and all workspace tests passed.
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=the
  lowering is direct two's-complement bitwise IR and introduces no trap or wrap rule.
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0043 and
  ADR-0045 compliance confirmed.
- 2026-07-11 main_task=main phase=ci result=pass evidence=formatter; Clippy;
  all workspace tests; focused bitwise validator. handoff=commit

## Required Outputs

- Authority read: ADR-0043, ADR-0045, ADR-0046.
- Files changed: backend, backend tests, task evidence, review, soundness
  report, and validator.
- Tests written before implementation and expected failure: bitwise operations
  initially returned `UnsupportedInstruction`.
- Validation commands and results: all required gates passed.
- Open questions: none.
- Remaining risk and next main-task action: checked shifts and exponentiation.
