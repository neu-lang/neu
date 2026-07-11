# Task: M0035-010 Primitive HIR Operators

## Task Metadata

- Task ID: `M0035-010`
- Milestone: `M0035`
- Milestone File: `docs/milestones/M0035-primitive-runtime-support.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Preserve ADR-0059 primitive unary and binary operators in HIR after source
typing, including Bool logical/equality and Float/Byte operator forms.

## Authority Extract

- ADR-0044 requires typed, ordered, source-mapped HIR expressions.
- ADR-0059 defines primitive operator families.
- M0035-009 defines the accepted type-checking results.

## Scope

- Add HIR unary `Not` and binary logical/comparison operators.
- Map parsed primitive operators into HIR.
- Lower checked-source unary and binary primitive expressions into HIR.
- Add focused representation and checked-source tests.

## Out Of Scope

- MIR operation variants or backend lowering.
- Short-circuit control-flow lowering.
- Primitive operation runtime traps or conversions.

## Tests

- HIR represents `!`, `&&`, `||`, equality, and comparisons.
- Checked source lowers typed primitive operators in source order.
- Unsupported operator/type combinations remain rejected before HIR.

## Acceptance Criteria

- HIR never maps logical/comparison operations to an unrelated arithmetic op.
- Operand order and source spans are preserved.
- Existing Int HIR lowering remains passing.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=primitive
  operator typing exists, but HIR lacks logical/comparison variants and checked
  source has no unary lowering branch. handoff=test-first
- 2026-07-11 main_task=main phase=test-first result=fail evidence=the HIR
  operator variants did not exist and MIR matches became explicitly incomplete.
  handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=HIR
  preserves logical/comparison and Not operators; unsupported MIR fallback is
  rejected explicitly. handoff=ordinary-tests
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=the
  focused HIR operator model test passed. handoff=adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=Not
  is not mapped to bitwise complement and comparisons are not arithmetic.
  handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0044,
  ADR-0059, SPEC.md, M0035, and task scope are aligned. handoff=ci

## Required Outputs

- Authority read: ADR-0044, ADR-0059, M0035-009, HIR, parser, and tests.
- Files changed: this task, HIR and MIR operator model/lowering, the M0028
  unary deferral boundary, focused tests, review evidence, soundness evidence,
  and the task validator.
- Tests written before implementation and expected pre-implementation failure:
  primitive HIR operator tests must fail because the variants/mapping do not
  exist.
- Validation commands and results recorded in this task's execution log.
- Open questions or `none`.
- Remaining risk and the next main-task action.
- 2026-07-11 main_task=main phase=ci result=pass evidence=cargo fmt --all
  --check; M0035-009 validator; M0035-010 validator; cargo clippy
  --workspace --all-targets -- -D warnings; cargo test --workspace
  --all-targets (367 passed); git diff --check. A regression in the legacy
  M0028 boundary was fixed so only accepted integer unary operators are
  executable there and `!` remains deferred. handoff=commit

## Open Questions

none

## Remaining Risk

MIR operation variants, backend lowering, and executable smoke coverage remain
subsequent M0035 tasks. Next main-task action: commit locally.
