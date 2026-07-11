# Task: M0035-013 Primitive Type-Checker Integration

## Task Metadata

- Task ID: `M0035-013`
- Milestone: `M0035`
- Milestone File: `docs/milestones/M0035-primitive-runtime-support.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Integrate the accepted Bool, Float, and Byte operator typing rules into the
executable-core source pipeline.

## Authority Extract

- ADR-0059 defines exact primitive operator families and no implicit numeric
  conversion.
- M0035-009 defines the primitive operator typing helper.
- M0028 remains the authority for the existing Int executable subset.

## Scope

- Invoke primitive operator typing from executable-core checking.
- Preserve exact operand and result types in the merged report.
- Ensure accepted primitive operators are not reported as deferred.
- Preserve diagnostics for invalid primitive operand types.

## Out Of Scope

- HIR/MIR/backend changes.
- Short-circuit control-flow lowering.
- Implicit conversions, casts, or new primitive operators.

## Tests

- Source Bool `!`, logical operators, and equality type-check through the core
  pipeline.
- Source Float arithmetic/comparison type-check through the core pipeline.
- Source Byte arithmetic and bitwise operations type-check through the core
  pipeline.
- Invalid mixed primitive operations produce type diagnostics rather than
  deferred-operator diagnostics.

## Acceptance Criteria

- A valid source primitive operator expression receives its result type in the
  executable-core report.
- Invalid primitive operations remain rejected with a concrete type diagnostic.
- Existing Int executable-core behavior and deferred non-primitive behavior
  remain passing.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=the
  primitive helper is currently isolated from `type_m0028_executable_core_in`.
  handoff=test-first
- 2026-07-11 main_task=main phase=test-first result=pending evidence=source
  integration tests are being added before type-core wiring. handoff=
  implementation
- 2026-07-11 main_task=main phase=test-first result=fail evidence=valid
  primitive source expressions received no result types and were reported as
  integer mismatches or deferred operators. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=the
  primitive checker is merged into executable-core typing, while the M0028
  integer checker filters Float and Byte operands. handoff=ordinary-tests
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=valid,
  invalid, and legacy Boolean/equality type-check tests passed. handoff=
  adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=no
  implicit conversion or deferred-error masking was introduced. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0059,
  M0035-009, M0028, and task scope are aligned. handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=cargo fmt --all;
  M0035-012 and M0035-013 validators; cargo clippy --workspace --all-targets
  -- -D warnings; cargo test --workspace --all-targets (372 passed); git diff
  --check. handoff=commit

## Open Questions

none

## Remaining Risk

Byte contextual expected-type propagation and short-circuit CFG lowering remain
subsequent M0035 tasks. Next main-task action: commit locally.

## Required Outputs

- Authority read: ADR-0059, M0035-009, M0028 type-checker source, and tests.
- Files changed: this task, type-checker source, focused tests, validator,
  review, and soundness evidence.
- Tests written before implementation and expected pre-implementation failure:
  valid primitive source expressions do not receive result types because the
  primitive helper is not called by executable-core typing.
- Validation commands and results recorded in this task's execution log.
- Open questions or `none`.
- Remaining risk and the next main-task action.
