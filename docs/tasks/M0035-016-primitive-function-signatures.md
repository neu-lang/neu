# Task: M0035-016 Primitive Function Signatures

## Task Metadata

- Task ID: `M0035-016`
- Milestone: `M0035`
- Milestone File: `docs/milestones/M0035-primitive-runtime-support.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Allow the checked-source function-signature phase to represent Bool, Unit,
Float, and Byte parameters and return types alongside Int.

## Authority Extract

- ADR-0058 defines function signatures and exact parameter/return types.
- ADR-0059 defines the four primitive identities and no implicit conversion.
- M0035-001 defines their TypeId identities in the module-owned arena.

## Scope

- Recognize the four primitive type names in function return annotations.
- Recognize them in parameter annotations.
- Construct signatures with exact primitive TypeIds from the caller-owned arena.
- Preserve existing unsupported-type filtering.

## Out Of Scope

- Function call type checking or argument lowering.
- Local reads, ownership, or parameter MIR lowering.
- Numeric conversion or overload resolution.

## Tests

- Bool, Unit, Float, and Byte functions produce signatures.
- Primitive parameter lists preserve order and exact TypeIds.
- Existing Int signatures and shared-arena behavior remain passing.
- Unsupported non-primitive annotations remain excluded.

## Acceptance Criteria

- All four requested primitive types can appear in signature position.
- The signature TypeIds come from the supplied TypeArena.
- No implicit conversion or type substitution is performed.
- Existing Int signature tests remain passing.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=function
  signature typing currently recognizes only Int, blocking primitive source
  pipeline entry. handoff=test-first
- 2026-07-11 main_task=main phase=test-first result=pending evidence=primitive
  signature tests are being added before widening the recognizer. handoff=
  implementation
- 2026-07-11 main_task=main phase=test-first result=fail evidence=the
  four-primitive signature fixture produced no signatures because the phase
  recognized only Int. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=exact
  primitive TypeIds are resolved for returns and ordered parameters, while
  String and Null remain excluded. handoff=ordinary-tests
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=the new
  primitive signature test, legacy signature test, and complete type-checker
  suite (106 tests) passed. handoff=adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=no
  conversion, promotion, or unsupported runtime type admission was introduced.
  handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0058,
  ADR-0059, M0035-001, and task scope are aligned. handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=cargo fmt --all
  --check; M0035-015 and M0035-016 validators; cargo clippy --workspace
  --all-targets -- -D warnings; cargo test --workspace --all-targets (376
  passed); git diff --check. handoff=commit

## Open Questions

none

## Remaining Risk

Function call type checking/lowering and end-to-end primitive execution remain
subsequent M0035 tasks. Next main-task action: commit locally.

## Required Outputs

- Authority read: ADR-0058, ADR-0059, M0035-001, signature source, and tests.
- Files changed: this task, type-checker source/tests, validator, review, and
  soundness evidence.
- Tests written before implementation and expected pre-implementation failure:
  only the Int signature is recognized from a four-primitive source fixture.
- Validation commands and results recorded in this task's execution log.
- Open questions or `none`.
- Remaining risk and the next main-task action.
