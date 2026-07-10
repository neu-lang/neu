# Task: M0028-011 Module-Wide Type Identity

## Task Metadata

- Task ID: `M0028-011`
- Milestone: `M0028`
- Milestone File: `docs/milestones/M0028-executable-expression-frontend-completion.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

## Goal

Allow executable signature typing to append into a caller-owned module
TypeArena, as required by ADR-0052.

## Authority Extract

- ADR-0025 defines module source-file membership.
- ADR-0027 defines type identity.
- ADR-0052 requires one TypeArena per module compilation.

## Scope

- Add a caller-owned function-signature typing API.
- Reuse existing bootstrap primitive identities across source files.
- Preserve the isolated convenience API for existing callers.

## Out Of Scope

- Direct-call resolution, argument checks, recursion, call result typing, HIR,
  MIR, backend, runtime, and linker work.

## Test-First Gate

- Test: two parsed source files append `Int` signatures into one TypeArena.
- Expected initial result: `fail`; caller-owned API is absent.

## Execution Log

- 2026-07-11 main_task=main phase=semantic-resolution result=pass evidence=ADR-0052 accepted module-wide TypeArena identity. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=caller-owned signature API absent. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=shared module primitive identities are reused. handoff=validation
- 2026-07-11 main_task=main phase=focused-validation result=pass evidence=two signature tests pass. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=format, Clippy, 289 workspace tests, and module identity validator passed. handoff=review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=separate source signatures share one caller-owned module arena. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=scope is limited to ADR-0052 type identity. handoff=commit
