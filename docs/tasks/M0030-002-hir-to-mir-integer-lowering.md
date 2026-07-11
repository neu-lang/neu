# Task: M0030-002 HIR To MIR Integer Lowering

## Task Metadata

- Task ID: `M0030-002`
- Milestone: `M0030`
- Milestone File: `docs/milestones/M0030-mir-design-and-lowering.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

## Goal

Lower checked HIR integer literals, arithmetic, direct calls, and explicit
returns into backend-independent MIR blocks and temporaries.

## Authority Extract

- ADR-0043 `Int` runtime operations.
- ADR-0044 HIR source mapping and typed operands.
- ADR-0045 MIR instructions, calls, returns, traps, and source mapping.

## Scope

- Lower approved HIR integer/direct-call expressions in operand order.
- Produce one entry block per straight-line function with a return terminator.
- Preserve HIR source spans on every MIR instruction and terminator.

## Out Of Scope

- Local storage, branches, cleanup insertion, optimization, backend code,
  runtime, and linker work.

## Test-First Gate

- Test: an HIR helper arithmetic function and direct-call main lower to ordered
  MIR instructions and return terminators with original spans.
- Expected initial result: `fail`; no HIR-to-MIR lowering API exists.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0030-001 provides backend-independent MIR forms. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=HIR-to-MIR lowering API is absent. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=straight-line typed HIR lowers to ordered MIR instructions and return terminators. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets. handoff=adversarial
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=unsupported HIR expressions produce no MIR; docs/tasks/soundness/M0030-002-soundness.md. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0045 compliance confirmed; docs/tasks/reviews/M0030-002-review.md. handoff=commit
- 2026-07-11 main_task=main phase=ci result=pass evidence=workspace validation and docs/tests/m0030-hir-to-mir-integer-lowering.sh passed. handoff=commit
