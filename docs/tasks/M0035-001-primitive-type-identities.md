# Task: M0035-001 Primitive Type Identities

## Task Metadata

- Task ID: `M0035-001`
- Milestone: `M0035`
- Milestone File: `docs/milestones/M0035-primitive-runtime-support.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Extend the compiler's primitive type registry and primitive annotation lookup
to include the ADR-0059 `Float` and `Byte` identities while preserving the
existing `Bool`, `Int`, `String`, `Unit`, and `Null` identities.

## Authority Extract

- ADR-0059 defines `Float` as binary64 and `Byte` as unsigned eight-bit.
- ADR-0027 defines primitive type identity and exact assignment compatibility.
- Existing TypeArena ownership and identity behavior remain unchanged.

## Scope

- Add `Float` and `Byte` primitive identities.
- Register them in the module-owned primitive arena.
- Recognize `Float` and `Byte` type annotation names.
- Add focused type identity and annotation tests.

## Out Of Scope

- Float/byte literal lexing or parsing.
- Primitive operations, conversions, HIR, MIR, backend lowering, or ABI work.
- Changes to `main` or existing `Int` semantics.

## Tests

- Annotated `Float` and `Byte` locals receive exact primitive signatures.
- The primitive registry contains seven identities in stable order.
- Unknown type names remain unresolved.

## Acceptance Criteria

- `Float` and `Byte` resolve only in accepted primitive annotation positions.
- Existing primitive TypeIds remain stable in their current insertion order.
- No implicit conversion or runtime representation is added in this task.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=ADR-0059
  accepts Float and Byte identities but current registry supports neither.
  handoff=test-first
- 2026-07-11 main_task=main phase=test-first result=fail evidence=the new
  Float/Byte assertions failed because the primitive variants did not exist.
  handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=Float
  and Byte are registered after the existing primitive identities and resolve
  in annotation positions. handoff=ordinary-tests
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=the
  compiler type-check suite passed 98 tests. handoff=adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=no
  conversion or runtime lowering path was widened. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0059,
  SPEC.md, and M0035 scope are aligned. handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=formatter, primitive
  planning and identity validators, Clippy with warnings denied, workspace
  tests, and diff checks passed. handoff=commit

## Required Outputs

- Authority read: ADR-0027, ADR-0059, M0035, and SPEC.md.
- Files changed: this task, primitive type registry, and focused tests.
- Tests written before implementation and expected pre-implementation failure:
  Float/Byte annotation assertions must fail because the names are unresolved.
- Validation commands and results: `cargo fmt --all --check`,
  `docs/tests/m0035-primitive-planning.sh`,
  `docs/tests/m0035-001-primitive-identities.sh`, Clippy with warnings denied,
  workspace tests, and `git diff --check` passed.
- Open questions or `none`.
- Remaining risk and the next main-task action.
- Remaining risk: literals and runtime lowering for Float and Byte remain
  subsequent M0035 tasks. Next main-task action: commit locally.
