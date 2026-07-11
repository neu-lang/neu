# Task: M0035-011 Primitive MIR Operations

## Task Metadata

- Task ID: `M0035-011`
- Milestone: `M0035`
- Milestone File: `docs/milestones/M0035-primitive-runtime-support.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Preserve the accepted primitive operation families in MIR without collapsing
comparisons or Boolean negation into unrelated arithmetic operations.

## Authority Extract

- ADR-0045 defines typed MIR values, operations, traps, and source mapping.
- ADR-0059 defines Bool, Float, and Byte operation families and exact types.
- M0035-010 requires HIR to preserve the parsed operator identities.

## Scope

- Add MIR operation forms for primitive comparisons and Boolean negation.
- Lower accepted HIR primitive operations into those MIR forms.
- Preserve operand order, result identity, and source spans.
- Keep arithmetic operations available for Int, Float, and Byte values.

## Out Of Scope

- Cranelift instruction selection.
- Short-circuit CFG lowering for `&&` and `||`.
- Runtime trap implementation and target-specific ABI changes.
- Type-checker integration beyond the existing accepted HIR input contract.

## Tests

- HIR `!` lowers to a distinct MIR logical-not operation.
- HIR equality and ordered comparisons lower to a distinct MIR comparison
  operation with preserved operands.
- Float and Byte arithmetic HIR lowers to the existing arithmetic operation
  with exact source mapping.
- Unsupported logical short-circuit operations remain explicit errors until
  their CFG contract is implemented.

## Acceptance Criteria

- MIR does not represent comparison or logical-not operations as arithmetic.
- Every new MIR operation retains output identity, operand order, and span.
- Existing Int MIR lowering and tests remain passing.
- Unsupported operations fail explicitly rather than being reinterpreted.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0035-010
  established HIR operator identities, while MIR still rejects comparisons and
  logical `Not`. handoff=test-first
- 2026-07-11 main_task=main phase=test-first result=fail evidence=the new
  logical-not and comparison tests failed because MIR had no distinct
  operation forms. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=MIR now
  preserves logical-not and comparison identities; short-circuit lowering
  remains an explicit unsupported boundary. handoff=ordinary-tests
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=the MIR
  suite passed (10 tests) including the new primitive operation coverage.
  handoff=adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=the
  comparison and logical-not forms are distinct, and short-circuit operators
  still fail explicitly. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0045,
  ADR-0059, M0035-010, and task scope are aligned. handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=cargo fmt --all
  --check; M0035-010 and M0035-011 validators; cargo clippy --workspace
  --all-targets -- -D warnings; cargo test --workspace --all-targets (369
  passed); git diff --check. handoff=commit

## Open Questions

none

## Remaining Risk

Cranelift operation selection and short-circuit CFG lowering remain subsequent
M0035 tasks. Next main-task action: commit locally.

## Required Outputs

- Authority read: ADR-0045, ADR-0059, M0035-010, MIR source, and MIR tests.
- Files changed: this task, MIR source, focused MIR tests, validator, review,
  and soundness evidence.
- Tests written before implementation and expected pre-implementation failure:
  primitive comparison and logical-not lowering tests fail because MIR has no
  distinct operation forms.
- Validation commands and results recorded in this task's execution log.
- Open questions or `none`.
- Remaining risk and the next main-task action.
