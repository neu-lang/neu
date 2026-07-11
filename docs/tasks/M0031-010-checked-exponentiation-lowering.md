# Task: M0031-010 Checked Exponentiation Lowering

## Task Metadata

- Task ID: `M0031-010`
- Milestone: `M0031`
- Milestone File: `docs/milestones/M0031-cranelift-backend-smoke.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

## Goal

Lower MIR `Exponent` for bootstrap `Int` with checked runtime exponentiation.

## Authority Extract

- ADR-0043 requires non-negative exponents, checked intermediate overflow, and
  runtime trapping for negative exponents.
- ADR-0045 includes exponentiation and trap terminators in MIR.
- ADR-0046 limits the runtime value to signed `I64`.

## Scope

- Lower positive-exponent `Int` exponentiation through a backend loop or
  equivalent verified IR sequence.
- Trap negative exponents and checked multiplication overflow.

## Out Of Scope

- Optimization, floating point, arbitrary precision, calls, locals, objects,
  linking, and executable startup.

## Test-First Gate

- Test: `2 ** 3` in MIR reaches verified Cranelift IR with repeated checked
  multiplication and a negative-exponent trap path.
- Expected initial result: `fail`; exponentiation is unsupported.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=ADR-0043
  requires runtime exponent and overflow behavior not covered by existing
  backend operations. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=Exponent
  returned `UnsupportedInstruction`. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=backend
  emits a verified exponentiation loop, traps negative exponents with user trap
  2, and checks each multiplication for signed overflow. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=focused
  exponentiation backend test and validator passed; formatter, Clippy, and all
  workspace tests passed.
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=the
  loop rejects negative counts before multiplication, returns one for exponent
  zero, and checks the full signed multiplication result on every iteration.
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0043,
  ADR-0045, and ADR-0046 compliance confirmed; unsupported MIR fixture remains
  unsupported after the arithmetic expansion.
- 2026-07-11 main_task=main phase=ci result=pass evidence=formatter; Clippy;
  all workspace tests; focused exponentiation backend validator; diff check.
  handoff=commit

## Required Outputs

- Authority read: ADR-0043, ADR-0045, ADR-0046.
- Files changed: `crates/compiler/src/backend.rs`,
  `crates/compiler/tests/backend.rs`, this task record, the review report, the
  soundness report, and the focused validator.
- Tests written before implementation and expected failure: the backend test
  was added first and initially failed with `UnsupportedInstruction`.
- Validation commands and results: all required gates passed.
- Open questions: none.
- Remaining risk and next main-task action: M0031 still needs the remaining
  backend/bootstrap tasks before object emission in M0032.
