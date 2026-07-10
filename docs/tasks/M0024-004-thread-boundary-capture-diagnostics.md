# Task: M0024-004 Thread Boundary Capture Diagnostics

## Task Metadata

- Task ID: `M0024-004`
- Milestone: `M0024`
- Milestone File: `docs/milestones/M0024-thread-safety-capability-analysis.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`
- Owner main task: `main-task implementation`

## Objective

Implement ADR-0037 metadata-only thread boundary and capture records with
`missing_thread_capability` diagnostics.

## Authority Extract

- `docs/adr/ADR-0037-bootstrap-thread-capability-analysis.md`, “Recommended Choice”.
- `docs/SPEC.md`, “ADR-0037: Bootstrap Thread Capability Analysis”.
- `docs/milestones/M0024-thread-safety-capability-analysis.md`.
- `crates/compiler/src/thread.rs`.
- `crates/compiler/src/name_resolution.rs`.

## Scope

- Add thread boundary records.
- Add capture records containing capture node, local binding, captured type, and
  required capability.
- Add `missing_thread_capability` diagnostics with capture node, boundary node,
  captured binding, required capability, and captured type.
- Preserve boundary and capture order in diagnostic output.

## Out Of Scope

- Parser support for concurrency constructs.
- Building boundary records from source syntax.
- Synchronization APIs.
- Generic capability-bound enforcement.
- User-declared capability implementations.

## Required Tests

- `crates/compiler/tests/thread.rs`
- `docs/tests/m0024-thread-boundary-capture-diagnostics.sh`

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Send-only captures accept `String`.
- [x] Share captures reject `String`.
- [x] Nominal, generic, and missing type captures diagnose.
- [x] Diagnostics preserve capture primary node and boundary secondary node.
- [x] No examples update is required because no user-written syntax changes.

## Execution Log

- 2026-07-11 agent=Main phase=create-task result=pass evidence=Task and test contract created from ADR-0037. handoff=Test
- 2026-07-11 agent=Main phase=test-first result=fail evidence=cargo test -p compiler --test thread m0024_boundary failed because boundary/capture APIs did not exist. handoff=Implementer
- 2026-07-11 agent=Main phase=implementation result=pass evidence=Added ThreadBoundary, ThreadCapture, ThreadDiagnostic, and analyze_thread_boundaries. handoff=Review
- 2026-07-11 agent=Main phase=validation result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets; M0024 docs validators. handoff=Commit
