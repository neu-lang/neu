# Task: M0031-008 Checked Shift Lowering

## Task Metadata

- Task ID: `M0031-008`
- Milestone: `M0031`
- Milestone File: `docs/milestones/M0031-cranelift-backend-smoke.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

## Goal

Lower bootstrap `Int` left and arithmetic-right shifts with an explicit
runtime count check for ADR-0043's inclusive `0..63` range.

## Authority Extract

- ADR-0043 shift range, arithmetic-right-shift, and invalid-count trap rules.
- ADR-0045 checked `Int` operations in MIR.
- ADR-0046 bootstrap backend traps are runtime-defined implementation detail.

## Scope

- Lower `ShiftLeft` to `ishl` and `ShiftRight` to `sshr`.
- Emit an internal backend trap for counts outside `0..63` before either shift.

## Out Of Scope

- Exponentiation, unary operations, calls, locals, object emission, linking,
  and executable startup.

## Test-First Gate

- Test: chained left/right shift MIR emits `ishl`, `sshr`, and an invalid-count
  conditional trap.
- Expected initial result: `fail`; shifts are unsupported.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=ADR-0043
  requires an explicit runtime count check before shifts. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=shifts
  returned `UnsupportedInstruction`. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=an
  unsigned count comparison rejects both negative counts and counts above 63.
  handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=focused
  shift test and validator passed; formatter, Clippy, and all workspace tests passed.
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=the
  unsigned check traps before a host-dependent shift can occur.
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0043 and
  ADR-0045 compliance confirmed.
- 2026-07-11 main_task=main phase=ci result=pass evidence=formatter; Clippy;
  all workspace tests; focused shift validator. handoff=commit

## Required Outputs

- Authority read: ADR-0043, ADR-0045, ADR-0046.
- Files changed: backend, backend tests, task evidence, review, soundness
  report, and validator.
- Tests written before implementation and expected failure: shifts initially
  returned `UnsupportedInstruction`.
- Validation commands and results: all required gates passed.
- Open questions: none.
- Remaining risk and next main-task action: exponentiation.
