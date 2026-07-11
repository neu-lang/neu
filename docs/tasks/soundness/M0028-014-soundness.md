# Soundness Report: M0028-014

## Metadata

- Task ID: `M0028-014`
- Milestone: `M0028`
- Filed By: `main-task adversarial check`
- Date: `2026-07-11`
- Decision: `pass`

## Inputs Read

- Task: `docs/tasks/M0028-014-direct-call-core-integration.md`.
- Authority: `docs/SPEC.md`, ADR-0041, ADR-0051, and ADR-0052.
- Changed implementation: `crates/compiler/src/type_check.rs`.
- Tests: `crates/compiler/tests/type_check.rs`.

## Safety Invariants Checked

- [x] Only a source-qualified successful direct call receives an executable-core result type.
- [x] Invalid calls remain untyped and retain `DirectCallDeferred`.
- [x] No ownership, borrow, thread, coroutine, unsafe, or FFI behavior changed.

## Attacks Attempted

```text
Attack: Apply an invalid direct-call report to executable-core typing.
Expected result: No result type and the deferred diagnostic remains.
Actual result: No result type; DirectCallDeferred remains on the call node.
Source of truth: ADR-0041 and ADR-0051.
Outcome: pass
```

## Findings

None.

## Ambiguities

None.

## Decision

Pass.
