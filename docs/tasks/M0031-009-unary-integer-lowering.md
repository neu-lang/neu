# Task: M0031-009 Unary Integer Lowering

## Task Metadata

- Task ID: `M0031-009`
- Milestone: `M0031`
- Milestone File: `docs/milestones/M0031-cranelift-backend-smoke.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

## Goal

Lower MIR unary `Int` operations into verified Cranelift IR.

## Authority Extract

- ADR-0043 unary plus, negation, complement, and negation-overflow rules.
- ADR-0045 checked `Int` operations in MIR.
- ADR-0046 bootstrap `Int` ABI lowering.

## Scope

- Lower unary plus as identity, unary minus as checked `ineg`, and complement
  as `bnot`.
- Trap when negating `Int.MIN_VALUE`.

## Out Of Scope

- Exponentiation, calls, locals, object emission, linking, and executable startup.

## Test-First Gate

- Test: unary MIR emits `ineg`, `bnot`, and an overflow trap for negation.
- Expected initial result: `fail`; unary MIR is unsupported by the backend.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0030-005
  now transports typed unary operations into MIR. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=unary MIR
  returned `UnsupportedInstruction`. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=plus is
  identity, negation traps for `Int.MIN_VALUE`, and complement emits `bnot`.
  handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=focused
  unary backend test and validator passed; formatter, Clippy, and all workspace tests passed.
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=negation
  traps before `ineg` for the sole signed overflow value.
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0043 and
  ADR-0045 compliance confirmed.
- 2026-07-11 main_task=main phase=ci result=pass evidence=formatter; Clippy;
  all workspace tests; focused unary backend validator. handoff=commit

## Required Outputs

- Authority read: ADR-0043, ADR-0045, ADR-0046.
- Files changed: backend, backend tests, task evidence, review, soundness
  report, and validator.
- Tests written before implementation and expected failure: unary MIR initially
  returned `UnsupportedInstruction`.
- Validation commands and results: all required gates passed.
- Open questions: none.
- Remaining risk and next main-task action: exponentiation remains.
