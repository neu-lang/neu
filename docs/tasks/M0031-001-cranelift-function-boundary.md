# Task: M0031-001 Cranelift Function Boundary

## Task Metadata

- Task ID: `M0031-001`
- Milestone: `M0031`
- Milestone File: `docs/milestones/M0031-cranelift-backend-smoke.md`
- Specification: `docs/SPEC.md`
- Status: `pending`

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

- Deferred until M0030-004 completes the ADR-0055 HIR-to-MIR type-environment
  transport prerequisite. Then create a test that passes the owning TypeArena
  with a MIR `Int` function containing `IntConstant(42)` and `Return` and
  expects verified Cranelift IR with an `i64` result.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=ADR-0043,
  ADR-0045, and ADR-0046 authorize the narrow backend boundary. handoff=test
- 2026-07-11 main_task=main phase=ambiguity result=resolved evidence=raw
  TypeId runtime interpretation is prohibited; ADR-0055 defines explicit
  module TypeArena transport. handoff=prerequisite

## Required Outputs

- Authority read: ADR-0040, ADR-0043, ADR-0045, ADR-0046, ADR-0055.
- Files changed: pending.
- Tests written before implementation and expected failure: pending.
- Validation commands and results: pending.
- Open questions: none.
- Remaining risk and next main-task action: extend only accepted MIR operations
  after this verified boundary exists.
