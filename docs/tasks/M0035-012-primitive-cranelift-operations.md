# Task: M0035-012 Primitive Cranelift Operations

## Task Metadata

- Task ID: `M0035-012`
- Milestone: `M0035`
- Milestone File: `docs/milestones/M0035-primitive-runtime-support.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Lower the accepted primitive MIR arithmetic, comparison, and Boolean
negation operations to typed Cranelift instructions.

## Authority Extract

- ADR-0059 defines Bool `i8`, Float `f64`, Byte unsigned `i8`, and operation
  behavior.
- ADR-0046 defines Cranelift as the bootstrap backend and requires explicit
  rejection of unsupported runtime forms.
- M0035-011 defines the MIR operation forms consumed by this task.

## Scope

- Lower primitive arithmetic for Int, Float, and Byte according to MIR value
  types.
- Lower Bool logical negation and primitive equality/ordered comparisons.
- Normalize Boolean results to `i8` values `0` or `1`.
- Preserve existing Int overflow and shift traps.
- Add focused Cranelift IR tests and negative operation coverage.

## Out Of Scope

- Short-circuit CFG lowering for `&&` and `||`.
- Function parameters, calls, linker startup, or executable runtime changes.
- New primitive conversions or implicit promotions.

## Tests

- Float arithmetic emits `fadd`, `fsub`, `fmul`, and `fdiv`.
- Byte operations use unsigned `i8` semantics and checked boundaries.
- Bool negation and primitive comparisons produce normalized Boolean results.
- Existing Int backend operation tests remain passing.

## Acceptance Criteria

- Cranelift does not apply integer instructions to Float values.
- Boolean-producing operations lower to normalized `i8` results.
- Byte checked arithmetic cannot silently wrap past `0..255`.
- Float division and NaN behavior are not converted into integer traps.
- Unsupported MIR operations remain explicit errors.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0035-011
  now provides explicit comparison and logical-not MIR operations. handoff=
  test-first
- 2026-07-11 main_task=main phase=test-first result=pending evidence=backend
  tests are being added before Cranelift implementation. handoff=implementation
- 2026-07-11 main_task=main phase=test-first result=fail evidence=the backend
  test reached integer lowering for a Float value and failed verification;
  new MIR operation forms were also unsupported. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=backend
  dispatches arithmetic and comparisons by primitive value type, normalizes
  Bool results, and checks Byte arithmetic boundaries. handoff=ordinary-tests
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=the
  focused primitive backend test and complete backend suite passed (13 tests).
  handoff=adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=the
  implementation keeps Float away from integer traps, uses unsigned Byte
  comparisons/shifts, and rejects unsupported MIR forms. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0046,
  ADR-0059, M0035-011, and task scope are aligned. handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=cargo fmt --all
  --check; M0035-011 and M0035-012 validators; cargo clippy --workspace
  --all-targets -- -D warnings; cargo test --workspace --all-targets (370
  passed); git diff --check. handoff=commit

## Open Questions

none

## Remaining Risk

Short-circuit CFG lowering and end-to-end primitive executable coverage remain
subsequent M0035 tasks. Next main-task action: commit locally.

## Required Outputs

- Authority read: ADR-0046, ADR-0059, M0035-011, backend source, and backend
  tests.
- Files changed: this task, backend source, focused backend tests, validator,
  review, and soundness evidence.
- Tests written before implementation and expected pre-implementation failure:
  primitive operation IR tests fail because the backend rejects new MIR forms.
- Validation commands and results recorded in this task's execution log.
- Open questions or `none`.
- Remaining risk and the next main-task action.
