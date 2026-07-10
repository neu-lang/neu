# Task: M0028-012 Module Executable Expression Typing

## Task Metadata

- Task ID: `M0028-012`
- Milestone: `M0028`
- Milestone File: `docs/milestones/M0028-executable-expression-frontend-completion.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

## Goal

Allow executable expression typing to use the caller-owned module TypeArena so
direct-call arguments share the ADR-0052 identity domain.

## Authority Extract

- ADR-0027 primitive expression typing.
- ADR-0041 argument compatibility.
- ADR-0052 module-wide type identity.

## Dependencies

- M0028-011 completion.

## Scope

- Add caller-owned executable-core typing that records `Int` argument facts in
  the module arena.
- Preserve the existing isolated executable-core convenience API.

## Out Of Scope

- Call resolution, arity/type diagnostics, recursion, and call result typing.

## Validation

- Test-first result: focused caller-owned executable-core API test was absent
  before implementation.
- Final result: formatting, Clippy, 290 workspace tests, and
  `docs/tests/m0028-module-expression-typing.sh` passed.

## Files Changed

- `crates/compiler/src/type_check.rs`
- `crates/compiler/tests/type_check.rs`
- `docs/tasks/M0028-012-direct-call-checker.md`
- `docs/tests/m0028-module-expression-typing.sh`

## Next Action

Implement the direct-call checker in M0028-013 using module-wide signatures
and expression types.
