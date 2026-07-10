# Task: M0024-003 Thread Capability Derivation

## Task Metadata

- Task ID: `M0024-003`
- Milestone: `M0024`
- Milestone File: `docs/milestones/M0024-thread-safety-capability-analysis.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`
- Owner main task: `main-task implementation`

## Objective

Implement the ADR-0037 bootstrap `Send` and `Share` capability derivation over
existing `TypeArena` records.

## Authority Extract

- `docs/adr/ADR-0037-bootstrap-thread-capability-analysis.md`, “Recommended Choice”.
- `docs/SPEC.md`, “ADR-0037: Bootstrap Thread Capability Analysis”.
- `docs/milestones/M0024-thread-safety-capability-analysis.md`.
- `crates/compiler/src/types.rs`.

## Scope

- Add a thread-capability module.
- Represent `Send` and `Share`.
- Classify primitive, nullable, nominal, generic, missing, and unsupported type
  categories according to ADR-0037.
- Export the module from the compiler crate.

## Out Of Scope

- Boundary and capture records.
- Missing-capability diagnostics.
- Parser support for concurrency constructs.
- Generic capability-bound enforcement.
- Synchronization APIs.

## Required Tests

- `crates/compiler/tests/thread.rs`
- `docs/tests/m0024-thread-capability-derivation.sh`

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] `Bool`, `Int`, `Unit`, and `Null` satisfy `Send` and `Share`.
- [x] `String` satisfies `Send` but not `Share`.
- [x] Nullable types inherit capability satisfaction from their base type.
- [x] Nominal, generic, missing, and unsupported type categories satisfy no
  capability in M0024.
- [x] No examples update is required because no user-written syntax changes.

## Execution Log

- 2026-07-11 agent=Main phase=create-task result=pass evidence=Task and test contract created from ADR-0037. handoff=Test
- 2026-07-11 agent=Main phase=test-first result=fail evidence=cargo test -p compiler --test thread failed because compiler::thread did not exist. handoff=Implementer
- 2026-07-11 agent=Main phase=implementation result=pass evidence=Added ThreadCapability and satisfies_thread_capability. handoff=Review
- 2026-07-11 agent=Main phase=validation result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets; M0024 docs validators. handoff=Commit
