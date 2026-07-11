# Task: M0030-001 Bootstrap MIR Model

## Task Metadata

- Task ID: `M0030-001`
- Milestone: `M0030`
- Milestone File: `docs/milestones/M0030-mir-design-and-lowering.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

## Goal

Define the backend-independent MIR data model required by ADR-0045, without
lowering HIR or using backend APIs.

## Authority Extract

- ADR-0043 bootstrap `Int` runtime operations.
- ADR-0045 MIR runtime contract and cleanup boundary.
- ADR-0044 HIR source-mapping contract.

## Scope

- Define stable MIR function, value, local, block, and temporary identities.
- Define ordered instructions, terminators, source spans, and cleanup boundary.
- Model constants, local load/store, checked arithmetic, direct calls, branches,
  returns, and traps.

## Out Of Scope

- HIR-to-MIR lowering, optimization, Cranelift, ABI, object emission, runtime,
  and linker work.

## Test-First Gate

- Test: a MIR function preserves ordered parameters, locals, temporaries,
  instructions, terminator, source spans, and an empty cleanup boundary.
- Expected initial result: `fail`; no MIR module exists.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0029 checked fixture lowering is complete and ADR-0045 is accepted. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=MIR module and data model are absent. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=backend-independent MIR preserves ordered instructions, terminators, spans, and cleanup boundary. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets. handoff=adversarial
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=cleanup remains a non-semantic reserved boundary and traps are explicit; docs/tasks/soundness/M0030-001-soundness.md. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0045 backend independence confirmed; docs/tasks/reviews/M0030-001-review.md. handoff=commit
- 2026-07-11 main_task=main phase=ci result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets; docs/tests/m0030-mir-model.sh. handoff=commit
