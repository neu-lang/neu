# Task: M0030-004 Type Environment Transport

## Task Metadata

- Task ID: `M0030-004`
- Milestone: `M0030`
- Milestone File: `docs/milestones/M0030-mir-design-and-lowering.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

## Goal

Require the owning module `TypeArena` at HIR-to-MIR lowering and validate that
all runtime-facing bootstrap types resolve to `Int` through that arena.

## Authority Extract

- ADR-0043 bootstrap `Int` runtime semantics.
- ADR-0044 HIR typed-expression and function-result facts.
- ADR-0045 MIR runtime type facts.
- ADR-0052 TypeId requires its owning module TypeArena.
- ADR-0055 type-environment transport at typed lowering boundaries.

## Scope

- Add the explicit `TypeArena` companion input to HIR-to-MIR lowering.
- Reject missing, foreign, and non-`Int` runtime-facing type identities.
- Cover successful `Int` and rejected unresolved-type cases.

## Out Of Scope

- Backend code, Cranelift, type inference, new language types, ABI lowering,
  local lowering, and control-flow lowering.

## Test-First Gate

- Test: an `Int` HIR function lowers with its owning TypeArena, while the same
  HIR fails when passed an arena that lacks the referenced type identity.
- Expected initial result: `fail`; HIR-to-MIR lowering accepts no TypeArena.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=ADR-0055
  resolves the M0031 type-environment ambiguity. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=HIR-to-MIR
  lowering accepted one argument and had no unsupported-runtime-type error.
  handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=the
  lowering boundary requires the owning arena and resolves runtime-facing types
  before emitting MIR. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=focused
  type-environment test and validator passed; formatter, Clippy, and all
  workspace tests passed before the final foreign-arena assertion was added.
  handoff=adversarial
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=raw
  TypeId values are never interpreted; missing and foreign arenas are rejected.
  `docs/tasks/soundness/M0030-004-soundness.md`. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0052 and
  ADR-0055 compliance confirmed; `docs/tasks/reviews/M0030-004-review.md`.
  handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=`git diff --check`;
  formatter; Clippy; all workspace tests; focused validator. handoff=commit

## Required Outputs

- Authority read: ADR-0043, ADR-0044, ADR-0045, ADR-0052, ADR-0055.
- Files changed: `crates/compiler/src/mir.rs`, `crates/compiler/tests/mir.rs`,
  task evidence, review, soundness report, and validator.
- Tests written before implementation and expected failure: the focused test
  failed because lowering had no TypeArena parameter or unsupported-runtime
  type error.
- Validation commands and results: all ordinary, adversarial, review, CI, and
  focused documentation-validator gates passed.
- Open questions: none.
- Remaining risk and next main-task action: M0031 backend boundary after this
  prerequisite completes.
