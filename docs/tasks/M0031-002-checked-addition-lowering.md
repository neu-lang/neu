# Task: M0031-002 Checked Addition Lowering

## Task Metadata

- Task ID: `M0031-002`
- Milestone: `M0031`
- Milestone File: `docs/milestones/M0031-cranelift-backend-smoke.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

## Goal

Lower MIR `Add` into Cranelift `I64` addition with an explicit integer-overflow
trap, as required by ADR-0043.

## Authority Extract

- ADR-0043 signed 64-bit `Int` and runtime overflow traps.
- ADR-0045 checked `Int` arithmetic in MIR.
- ADR-0046 bootstrap `Int` ABI lowering.
- ADR-0055 TypeArena-backed runtime type resolution.

## Scope

- Lower ordered `MirArithmetic::Add` operands to `iadd`.
- Detect signed addition overflow and emit an integer-overflow trap.
- Preserve unsupported lowering for every other arithmetic operation.

## Out Of Scope

- Subtraction, multiplication, division, remainder, exponentiation, bitwise,
  shifts, calls, locals, objects, linking, and executable startup.

## Test-First Gate

- Test: two `Int` constants, `Add`, and `Return` produce `iadd` and an
  `int_ovf` trap in verified Cranelift IR.
- Expected initial result: `fail`; checked arithmetic is rejected as an
  unsupported instruction.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=ADR-0043
  requires trapping rather than wrapping runtime addition. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=checked
  addition returned `UnsupportedInstruction`. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=signed
  addition emits a sign-change overflow check and `INTEGER_OVERFLOW` trap.
  handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=focused
  checked-addition test and validator passed; formatter, Clippy, and all
  workspace tests passed. handoff=adversarial
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=the
  generated IR checks the signed overflow condition and traps with `int_ovf`.
  `docs/tasks/soundness/M0031-002-soundness.md`. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0043 and
  ADR-0045 compliance confirmed; `docs/tasks/reviews/M0031-002-review.md`.
  handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=formatter; Clippy;
  all workspace tests; focused checked-addition validator. handoff=commit

## Required Outputs

- Authority read: ADR-0043, ADR-0045, ADR-0046, ADR-0055.
- Files changed: backend, backend tests, task evidence, review, soundness
  report, and validator.
- Tests written before implementation and expected failure: checked addition
  initially returned `UnsupportedInstruction`.
- Validation commands and results: all ordinary, adversarial, review, CI, and
  focused documentation-validator gates passed.
- Open questions: none.
- Remaining risk and next main-task action: lower the remaining accepted
  arithmetic operations independently.
