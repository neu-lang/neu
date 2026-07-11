# Task: M0035-004 Primitive HIR Model

## Task Metadata

- Task ID: `M0035-004`
- Milestone: `M0035`
- Milestone File: `docs/milestones/M0035-primitive-runtime-support.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Extend HIR to preserve Bool, Unit, Float, and Byte literal payloads and exact
types without losing source mapping or changing existing Int representation.

## Authority Extract

- ADR-0059 requires typed literal/value preservation through HIR.
- ADR-0044 requires source spans, expression types, and safety facts to survive
  HIR lowering.
- Existing HIR uses backend-independent expression kinds.

## Scope

- Add HIR literal variants and constructors for Bool, Unit, Float, and Byte.
- Preserve Float values by exact IEEE bit pattern.
- Add focused model tests for payloads, types, and spans.

## Out Of Scope

- Parser-to-HIR primitive lowering changes.
- MIR instruction design or lowering.
- Cranelift or ABI changes.
- Primitive operations, conversions, or control flow.

## Tests

- Each primitive HIR literal preserves its payload and exact TypeId.
- Float round-trips through its bit pattern, including signed zero.
- Unit has no fabricated payload.
- Existing Int HIR tests remain unchanged.

## Acceptance Criteria

- HIR can represent all four requested primitive literal forms.
- HIR remains backend-independent and source-mapped.
- No primitive value is reinterpreted as Int.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=HIR
  currently has only IntLiteral and ADR-0059 requires four additional typed
  literal forms. handoff=test-first
- 2026-07-11 main_task=main phase=test-first result=fail evidence=primitive HIR
  constructors and variants did not exist. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=Bool,
  Unit, Float-bit-pattern, and Byte HIR literals preserve type and span.
  handoff=ordinary-tests
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=the
  focused primitive HIR model test passed. handoff=adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=Unit
  has no payload and Float values are not coerced to Int. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0044,
  ADR-0059, SPEC.md, M0035, and task scope are aligned. handoff=ci

## Required Outputs

- Authority read: ADR-0044, ADR-0059, M0035, HIR source, and existing tests.
- Files changed: this task, HIR model, and focused tests.
- Tests written before implementation and expected pre-implementation failure:
  primitive HIR constructor tests must fail because the variants do not exist.
- Validation commands and results: primitive planning, identity, literal
  frontend, literal typing, and HIR validators, formatter, Clippy with warnings
  denied, workspace tests, and diff checks passed.
- Open questions or `none`.
- Remaining risk and the next main-task action.
- Remaining risk: parser-to-HIR and MIR/backend primitive lowering remain
  subsequent M0035 tasks. Next main-task action: commit locally.
