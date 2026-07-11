# Task: M0029-001 Bootstrap HIR Model

## Task Metadata

- Task ID: `M0029-001`
- Milestone: `M0029`
- Milestone File: `docs/milestones/M0029-hir-design-and-lowering.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

## Goal

Introduce the backend-independent, source-mapped HIR data model required by
ADR-0044, without AST lowering.

## Authority Extract

- ADR-0044 HIR runtime contract.
- ADR-0025 module/package identity.
- ADR-0027 type identity.
- ADR-0035 ownership facts.

## Scope

- Define HIR identities, source spans, typed expressions, functions, locals,
  direct calls, and explicit returns for the accepted executable subset.
- Represent required safety and unsupported-form markers.

## Out Of Scope

- AST lowering, parser changes, MIR, optimization, code generation, runtime,
  and linker work.

## Test-First Gate

- Test: HIR records preserve ordered typed function, local, expression, call,
  and return facts with source spans.
- Expected initial result: `fail`; no HIR module exists.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0028 completion checklist is complete and ADR-0044 is accepted. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=HIR module and model API are absent. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=backend-independent typed HIR preserves source-mapped executable facts without lowering. handoff=validation
- 2026-07-11 main_task=main phase=focused-validation result=pass evidence=cargo test -p compiler --test hir passed. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets; docs/tests/m0029-hir-model.sh. handoff=adversarial
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=HIR preserves explicit facts without introducing runtime semantics; docs/tasks/soundness/M0029-001-soundness.md. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0044 compliance confirmed; docs/tasks/reviews/M0029-001-review.md. handoff=commit
- 2026-07-11 main_task=main phase=ci result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets; docs/tests/m0029-hir-model.sh. handoff=commit
