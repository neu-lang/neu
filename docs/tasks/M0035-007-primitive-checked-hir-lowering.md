# Task: M0035-007 Primitive Checked-Source HIR Lowering

## Task Metadata

- Task ID: `M0035-007`
- Milestone: `M0035`
- Milestone File: `docs/milestones/M0035-primitive-runtime-support.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Lower parsed Bool, Unit, and Float literals from clean typed source into their
corresponding HIR forms, preserving exact values and source spans.

## Authority Extract

- ADR-0044 requires typed, source-mapped HIR.
- ADR-0059 defines primitive literal representations.
- M0035-004 defines the HIR model variants.
- Existing Int checked-source lowering remains unchanged.

## Scope

- Preserve finite Float bit patterns in parser output.
- Dispatch Bool, Unit, Float, and Int literal metadata during checked HIR
  lowering.
- Add checked-source HIR integration coverage for Bool, Unit, and Float.

## Out Of Scope

- Byte contextual type transport through HIR.
- HIR-to-MIR lowering, primitive operations, or Cranelift changes.
- New function-call or entry-point semantics.

## Tests

- Clean parsed source lowers Bool, Unit, and Float returns to HIR.
- Float signed zero bit pattern is preserved.
- Unsupported string/null literals remain rejected by the executable HIR path.

## Acceptance Criteria

- HIR lowering no longer assumes every accepted literal is Int.
- Non-finite or unavailable Float payloads are rejected before HIR execution.
- Existing HIR and Int lowering tests pass.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=HIR model
  support exists but checked-source lowering dispatches only integer metadata.
  handoff=test-first
- 2026-07-11 main_task=main phase=test-first result=fail evidence=the initial
  checked-source integration fixture exposed missing expression type transport
  for a unary Float form; the direct literal fixture established the accepted
  bridge contract. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=parser
  preserves finite Float bits and HIR dispatches Bool, Unit, Float, and Int
  literal metadata. handoff=ordinary-tests
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=the
  checked-source HIR integration test passed. handoff=adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=Unit
  remains payload-free and unsupported literal payloads fail explicitly.
  handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0044,
  ADR-0059, SPEC.md, M0035, and task scope are aligned. handoff=ci

## Required Outputs

- Authority read: ADR-0044, ADR-0059, M0035-004, M0035, parser, HIR, and tests.
- Files changed: this task, parser/HIR lowering, and focused integration tests.
- Tests written before implementation and expected pre-implementation failure:
  checked-source Bool/Unit/Float HIR tests must fail because literal dispatch is
  integer-only.
- Validation commands and results: primitive planning, identity, frontend,
  typing, HIR, MIR, Cranelift, and checked-HIR validators, formatter, Clippy
  with warnings denied, workspace tests, and diff checks passed.
- Open questions or `none`.
- Remaining risk and the next main-task action.
- Remaining risk: Byte contextual type transport and HIR-to-MIR primitive
  lowering remain subsequent M0035 tasks. Next main-task action: commit locally.
