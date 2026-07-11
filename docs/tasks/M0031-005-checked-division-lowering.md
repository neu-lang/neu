# Task: M0031-005 Checked Division Lowering

## Task Metadata

- Task ID: `M0031-005`
- Milestone: `M0031`
- Milestone File: `docs/milestones/M0031-cranelift-backend-smoke.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

## Goal

Lower MIR signed division using Cranelift's checked `sdiv` operation.

## Authority Extract

- ADR-0043 signed division truncates toward zero and traps on zero divisor or
  overflow.
- ADR-0045 checked `Int` arithmetic in MIR.
- ADR-0046 bootstrap `Int` ABI lowering.

## Scope

- Lower ordered `MirArithmetic::Divide` operands to Cranelift `sdiv`.
- Depend only on Cranelift's documented zero-divisor and signed-overflow trap
  behavior, which matches ADR-0043.

## Out Of Scope

- Remainder, exponentiation, bitwise, shifts, calls, locals, object emission,
  linking, and executable startup.

## Test-First Gate

- Test: two `Int` constants, `Divide`, and `Return` produce verified `sdiv` IR.
- Expected initial result: `fail`; division is unsupported.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=the
  selected Cranelift version documents traps for zero divisor and Min/-1.
  handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=division
  returned `UnsupportedInstruction`. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=`sdiv`
  carries the documented zero-divisor and signed-overflow traps. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=focused
  division test and validator passed; formatter, Clippy, and all workspace tests passed.
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=sdiv
  specifies traps for zero and Min/-1; no wrapping division path exists.
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0043 and
  ADR-0045 compliance confirmed.
- 2026-07-11 main_task=main phase=ci result=pass evidence=formatter; Clippy;
  all workspace tests; focused division validator. handoff=commit

## Required Outputs

- Authority read: ADR-0043, ADR-0045, ADR-0046.
- Files changed: backend, backend tests, task evidence, review, soundness
  report, and validator.
- Tests written before implementation and expected failure: division initially
  returned `UnsupportedInstruction`.
- Validation commands and results: all required gates passed.
- Open questions: none.
- Remaining risk and next main-task action: lower remainder independently.
