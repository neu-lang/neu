# Soundness Report: M0030-003

## Metadata

- Task ID: `M0030-003`
- Milestone: `M0030`
- Filed By: `main-task adversarial check`
- Date: `2026-07-11`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0030-003-mir-function-return-type.md`
- Milestone file: `docs/milestones/M0030-mir-design-and-lowering.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0041-bootstrap-function-call-and-return-semantics.md`
  - `docs/adr/ADR-0044-bootstrap-hir-runtime-contract.md`
  - `docs/adr/ADR-0045-bootstrap-mir-runtime-contract.md`
  - `docs/adr/ADR-0046-bootstrap-abi-and-calling-convention.md`
- Changed files:
  - `crates/compiler/src/mir.rs`
  - `crates/compiler/tests/mir.rs`
- Ordinary test results:
  - `cargo fmt --all --check`, Clippy, and all workspace tests passed.

## Safety Invariants Checked

- [x] Ownership cannot be bypassed.
- [x] Moved values cannot be reused.
- [x] Shared and exclusive borrows cannot conflict.
- [x] Borrowed data cannot outlive its owner.
- [x] Nullability refinements cannot be used after invalidation.
- [x] Thread send/share capabilities are enforced.
- [x] Coroutine scopes cannot outlive allowed ownership or borrow lifetimes.
- [x] Borrows across suspension are rejected unless proven safe by accepted semantics.
- [x] Unsafe and FFI boundaries do not weaken safe-code guarantees.
- [x] Diagnostics do not hide or misstate safety failures.

## Attacks Attempted

```text
Attack: Lower an `Int`-returning HIR function and lose its return type.
Expected result: MIR retains the exact declared `TypeId`.
Actual result: focused model and lowering tests observe the original `Int` type.
Source of truth: ADR-0045 and ADR-0046.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/compiler/tests/mir.rs`
- Tests run:
  - `cargo test -p compiler --test mir m0030_mir_function_preserves_declared_return_type`
- Result:
  - pass

## Findings

None.

## Ambiguities

None.

## Decision

Pass. The change transports an existing typed-HIR fact into MIR and introduces
no ownership, borrow, concurrency, unsafe, or runtime semantics.
