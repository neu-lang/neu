# Task: M0029-002 HIR Executable Expression Model

## Task Metadata

- Task ID: `M0029-002`
- Milestone: `M0029`
- Milestone File: `docs/milestones/M0029-hir-design-and-lowering.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

## Goal

Extend the HIR model with ADR-0042 executable local, unary, binary, and
assignment expression forms before AST lowering begins.

## Authority Extract

- ADR-0042 executable `Int` expression subset and evaluation order.
- ADR-0043 integer operation semantics.
- ADR-0044 typed source-mapped HIR and left-to-right operand order.

## Scope

- Model local reads and assignments.
- Model all accepted unary and binary `Int` operators with ordered operands.
- Preserve expression types and source spans.

## Out Of Scope

- AST lowering, control-flow lowering, unsupported forms, MIR, and backend
  code generation.

## Test-First Gate

- Test: HIR preserves ordered local, unary, binary, and assignment facts for
  every accepted executable operator.
- Expected initial result: `fail`; the HIR expression model has no such forms.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0029-001 establishes the base model; ADR-0044 requires the remaining executable expression forms. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=HIR local, unary, binary, and assignment model API is absent. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=accepted executable local, unary, binary, and assignment forms preserve typed ordered operands. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets. handoff=adversarial
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=explicit operands do not evaluate or weaken prior safety checks; docs/tasks/soundness/M0029-002-soundness.md. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0042 and ADR-0044 compliance confirmed; docs/tasks/reviews/M0029-002-review.md. handoff=commit
- 2026-07-11 main_task=main phase=ci result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets; docs/tests/m0029-hir-executable-expression-model.sh. handoff=commit
