# Task: M0029-004 Checked AST To HIR Lowering

## Task Metadata

- Task ID: `M0029-004`
- Milestone: `M0029`
- Milestone File: `docs/milestones/M0029-hir-design-and-lowering.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

## Goal

Lower one explicitly supplied checked executable source unit into typed,
source-mapped HIR, rejecting unresolved, diagnosed, or unsupported input.

## Dependencies

- M0029-003 executable AST traversal metadata.

## Authority Extract

- ADR-0041, ADR-0042, ADR-0044, ADR-0052, ADR-0053, and ADR-0054.

## Scope

- Define the checked executable-source input bundle.
- Lower checked literal, unary, binary, direct-call, and explicit-return
  expressions to HIR in source order.
- Reject input with diagnostics or unsupported-form markers.

## Out Of Scope

- Local binding identity lowering, re-running frontend analyses, MIR,
  optimization, backend, runtime, and linker work.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0029-001 through M0029-003 provide representation and traversal prerequisites. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=CheckedHirSource and lowering API are absent. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=clean typed integer and direct-call sources lower to source-mapped HIR; missing type facts are rejected. handoff=validation
- 2026-07-11 main_task=main phase=focused-validation result=pass evidence=cargo test -p compiler --test hir m0029_checked_source_lowers passed. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets. handoff=adversarial
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=unclean, missing-type, and unsupported input produce no HIR; docs/tasks/soundness/M0029-004-soundness.md. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0044 checked-input boundary confirmed; docs/tasks/reviews/M0029-004-review.md. handoff=commit
- 2026-07-11 main_task=main phase=ci result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets; docs/tests/m0029-checked-ast-hir-lowering.sh. handoff=commit
