# Task: M0031-006 Checked Remainder Lowering

## Task Metadata

- Task ID: `M0031-006`
- Milestone: `M0031`
- Milestone File: `docs/milestones/M0031-cranelift-backend-smoke.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

## Goal

Lower MIR signed remainder using Cranelift's checked `srem` operation.

## Authority Extract

- ADR-0043 remainder sign and zero-divisor trap semantics.
- ADR-0045 checked `Int` arithmetic in MIR.
- ADR-0046 bootstrap `Int` ABI lowering.

## Scope

- Lower ordered `MirArithmetic::Remainder` operands to Cranelift `srem`.
- Depend only on Cranelift's documented zero-divisor trap and dividend-sign
  result rule, which matches ADR-0043.

## Out Of Scope

- Exponentiation, bitwise, shifts, calls, locals, object emission, linking,
  and executable startup.

## Test-First Gate

- Test: two `Int` constants, `Remainder`, and `Return` produce verified `srem` IR.
- Expected initial result: `fail`; remainder is unsupported.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=the
  selected Cranelift version documents dividend-sign remainder and zero traps.
  handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=remainder
  returned `UnsupportedInstruction`. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=`srem`
  carries documented dividend-sign and zero-divisor behavior. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=focused
  remainder test and validator passed; formatter, Clippy, and all workspace tests passed.
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=srem
  specifies a zero-divisor trap and dividend-sign remainder behavior.
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0043 and
  ADR-0045 compliance confirmed.
- 2026-07-11 main_task=main phase=ci result=pass evidence=formatter; Clippy;
  all workspace tests; focused remainder validator. handoff=commit

## Required Outputs

- Authority read: ADR-0043, ADR-0045, ADR-0046.
- Files changed: backend, backend tests, task evidence, review, soundness
  report, and validator.
- Tests written before implementation and expected failure: remainder initially
  returned `UnsupportedInstruction`.
- Validation commands and results: all required gates passed.
- Open questions: none.
- Remaining risk and next main-task action: lower bitwise operations.
