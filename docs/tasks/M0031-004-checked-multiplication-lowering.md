# Task: M0031-004 Checked Multiplication Lowering

## Task Metadata

- Task ID: `M0031-004`
- Milestone: `M0031`
- Milestone File: `docs/milestones/M0031-cranelift-backend-smoke.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

## Goal

Lower MIR `Multiply` to signed `I64` multiplication with overflow trapping.

## Authority Extract

- ADR-0043 signed 64-bit `Int` and runtime overflow traps.
- ADR-0045 checked `Int` arithmetic in MIR.
- ADR-0046 bootstrap `Int` ABI lowering.
- ADR-0055 TypeArena-backed runtime type resolution.

## Scope

- Lower ordered `MirArithmetic::Multiply` operands to `imul`.
- Compare signed high-half multiplication output with the low-result sign
  extension and trap on mismatch.

## Out Of Scope

- Division, remainder, exponentiation, bitwise, shifts, calls, locals, object
  emission, linking, and executable startup.

## Test-First Gate

- Test: two `Int` constants, `Multiply`, and `Return` produce `imul`, `smulhi`,
  and `int_ovf` in verified Cranelift IR.
- Expected initial result: `fail`; multiplication is unsupported.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=ADR-0043
  requires runtime multiplication overflow to trap. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=multiplication
  returned `UnsupportedInstruction`. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=the
  signed product high half must equal the sign extension of the low half.
  handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=focused
  multiplication test and validator passed; formatter, Clippy, and all
  workspace tests passed. handoff=adversarial
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=the
  high-half check prevents silent signed multiplication overflow. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0043 and
  ADR-0045 compliance confirmed. handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=formatter; Clippy;
  all workspace tests; focused multiplication validator. handoff=commit

## Required Outputs

- Authority read: ADR-0043, ADR-0045, ADR-0046, ADR-0055.
- Files changed: backend, backend tests, task evidence, review, soundness
  report, and validator.
- Tests written before implementation and expected failure: multiplication
  initially returned `UnsupportedInstruction`.
- Validation commands and results: all ordinary, adversarial, review, CI, and
  focused documentation-validator gates passed.
- Open questions: none.
- Remaining risk and next main-task action: lower division and remainder.
