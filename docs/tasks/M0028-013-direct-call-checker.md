# Task: M0028-013 Direct Call Checker

## Task Metadata

- Task ID: `M0028-013`
- Milestone: `M0028`
- Milestone File: `docs/milestones/M0028-executable-expression-frontend-completion.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

## Goal

Validate package-scoped direct calls using typed signatures and unified
module-wide executable expression types.

## Authority Extract

- ADR-0041 direct-call semantics.
- ADR-0051 direct-call diagnostic contract.
- ADR-0052 module-wide type identity.

## Dependencies

- M0028-012 completion.

## Required Input Contract

Each source input to the checker must carry:

- its explicit `PackageNamespace`;
- its `ParseOutput`, including source-qualified AST spans;
- function signatures produced in the shared module `TypeArena`; and
- executable expression types produced in that same arena.

The checker must derive function and call identity from the source-qualified
declaration/call spans, not raw `AstNodeId` values across files.

## Test-First Gate

- Test files: `crates/compiler/tests/type_check.rs` and
  `docs/tests/m0028-direct-call-checker.sh`.
- Expected initial result: `fail`; no package-scoped direct-call checker API
  exists.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0028-010 through M0028-012 prerequisites are complete. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=package-scoped direct-call checker API was absent. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=direct calls resolve only to one same-package top-level function with a body. handoff=test
- 2026-07-11 main_task=main phase=regression-test result=pass evidence=three-function recursive cycle emits recursive_call_unsupported for each edge and records no result types. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets; docs/tests/m0028-direct-call-checker.sh. handoff=adversarial
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=three-function recursion produces three recursive_call_unsupported diagnostics and no call result types; docs/tasks/soundness/M0028-013-soundness.md. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0041 and ADR-0051 compliance confirmed; docs/tasks/reviews/M0028-013-review.md. handoff=next-task
- 2026-07-11 main_task=main phase=ci result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets; docs/tests/m0028-direct-call-checker.sh. handoff=commit
