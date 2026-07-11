# Task: M0035-015 Byte HIR Type Transport

## Task Metadata

- Task ID: `M0035-015`
- Milestone: `M0035`
- Milestone File: `docs/milestones/M0035-primitive-runtime-support.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Transport a contextually typed `Byte` literal into HIR as a `ByteLiteral`
instead of retaining its lexical `Int` type.

## Authority Extract

- ADR-0044 requires HIR to preserve exact typed expression facts.
- ADR-0059 defines contextual Byte literal typing and `u8` representation.
- M0035-014 accepts only direct in-range integer literal initializers for this
  contextual rule.

## Scope

- Add an explicit expression-type override operation to the type-check report.
- Override direct accepted Byte literal expression types to `Byte`.
- Ensure checked-source HIR lowering emits `ByteLiteral` with the correct type.
- Preserve existing literal type reporting for ordinary Int literals.

## Out Of Scope

- Byte binary-expression contextual propagation.
- Local reads, parameters, calls, MIR, or backend changes.
- General type inference or implicit conversions.

## Tests

- Checked source `const value: Byte = 255; return value`-equivalent literal
  lowering preserves Byte type and value where the current source subset allows.
- Ordinary Int literals remain Int typed.
- Out-of-range Byte literals remain rejected before HIR.

## Acceptance Criteria

- Accepted contextual Byte literals are represented in HIR as `ByteLiteral`.
- The HIR expression type is the Byte TypeId.
- No non-literal expression receives a contextual Byte override.
- Existing HIR primitive and integer literal tests remain passing.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0035-014
  accepts Byte literals in executable-core assignment checking, but HIR reads
  the original Int expression type. handoff=test-first
- 2026-07-11 main_task=main phase=test-first result=pending evidence=checked
  source HIR coverage is being added before the report transport change.
  handoff=implementation
- 2026-07-11 main_task=main phase=test-first result=fail evidence=checked
  Byte literal source lowered as IntLiteral because HIR lacked primitive type
  context. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=checked
  source lowering accepts an explicit Byte type identity and emits ByteLiteral
  only for that typed integer literal. handoff=ordinary-tests
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=the
  contextual Byte HIR regression passed. handoff=adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=no
  raw TypeId inference or non-literal reclassification was introduced.
  handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0044,
  ADR-0059, M0035-014, and task scope are aligned. handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=cargo fmt --all
  --check; M0035-014 and M0035-015 validators; cargo clippy --workspace
  --all-targets -- -D warnings; cargo test --workspace --all-targets (375
  passed); git diff --check. handoff=commit

## Open Questions

none

## Remaining Risk

Byte locals/parameters, short-circuit CFG lowering, and end-to-end executable
coverage remain subsequent M0035 tasks. Next main-task action: commit locally.

## Required Outputs

- Authority read: ADR-0044, ADR-0059, M0035-014, HIR source, and type-checker
  source/tests.
- Files changed: this task, type-check report/HIR tests, validator, review, and
  soundness evidence.
- Tests written before implementation and expected pre-implementation failure:
  checked Byte source remains an Int literal in HIR.
- Validation commands and results recorded in this task's execution log.
- Open questions or `none`.
- Remaining risk and the next main-task action.
