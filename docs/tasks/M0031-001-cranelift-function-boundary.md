# Task: M0031-001 Cranelift Function Boundary

## Task Metadata

- Task ID: `M0031-001`
- Milestone: `M0031`
- Milestone File: `docs/milestones/M0031-cranelift-backend-smoke.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

## Goal

Lower a typed, no-parameter MIR `Int` function containing an integer constant
and return into verified Cranelift IR for the current host smoke target.

## Authority Extract

- ADR-0040 bootstrap `main` returns `Int`.
- ADR-0043 bootstrap `Int` is signed 64-bit.
- ADR-0045 MIR preserves function return types, values, instructions, and
  terminator source mapping.
- ADR-0046 permits Cranelift platform-default calling convention on the current
  host target for internal Neu functions.
- ADR-0055 requires the owning module `TypeArena` to accompany runtime type
  lowering.

## Scope

- Add the pinned Cranelift code-generation and frontend dependencies.
- Add an internal backend boundary for lowering one literal-and-return MIR
  function to verified Cranelift IR.
- Resolve bootstrap `Int` return types through the owning module `TypeArena`
  and map them to Cranelift `I64`.
- Reject unsupported MIR instructions explicitly.

## Out Of Scope

- Arithmetic instruction lowering, direct calls, parameters, locals, object
  emission, linking, executable startup, target packs, and public ABI.

## Test-First Gate

- Test: a MIR `Int` function with `IntConstant(42)` and `Return`, paired with
  its owning TypeArena, produces verified Cranelift IR with an `i64` result and
  integer constant.
- Expected initial result: `fail`; no Cranelift dependency or lowering module
  exists.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=ADR-0043,
  ADR-0045, and ADR-0046 authorize the narrow backend boundary. handoff=test
- 2026-07-11 main_task=main phase=ambiguity result=resolved evidence=raw
  TypeId runtime interpretation is prohibited; ADR-0055 defines explicit
  module TypeArena transport. handoff=prerequisite
- 2026-07-11 main_task=main phase=prerequisite result=pass evidence=M0030-004
  now supplies the required TypeArena lowering boundary. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=the compiler
  exposes no backend module or Cranelift lowering API. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=the
  literal-and-return MIR slice lowers to verified host-default Cranelift IR.
  handoff=validation
- 2026-07-11 main_task=main phase=test-adjustment result=pass evidence=the
  Cranelift display format uses `-> i64`; the test asserts the target-independent
  signature marker rather than a printer-specific parenthesized form.
  handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=focused
  backend test and validator passed; formatter, Clippy, and all workspace tests
  passed. handoff=adversarial
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=the
  backend rejects non-Int types, unsupported instruction/terminator forms, and
  missing values; `docs/tasks/soundness/M0031-001-soundness.md`. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0043,
  ADR-0046, and ADR-0055 compliance confirmed;
  `docs/tasks/reviews/M0031-001-review.md`. handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=`git diff --check`;
  formatter; Clippy; all workspace tests; focused backend validator.
  handoff=commit

## Required Outputs

- Authority read: ADR-0040, ADR-0043, ADR-0045, ADR-0046, ADR-0055.
- Files changed: compiler manifest/lockfile, backend module/export, backend
  test, task evidence, review, soundness report, and validator.
- Tests written before implementation and expected failure: the backend import
  failed because no lowering boundary existed.
- Validation commands and results: all ordinary, adversarial, review, CI, and
  focused documentation-validator gates passed.
- Open questions: none.
- Remaining risk and next main-task action: extend only accepted MIR operations
  after this verified boundary exists.
