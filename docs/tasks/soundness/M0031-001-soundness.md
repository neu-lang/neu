# Soundness Report: M0031-001

## Metadata

- Task ID: `M0031-001`
- Milestone: `M0031`
- Filed By: `main-task adversarial check`
- Date: `2026-07-11`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0031-001-cranelift-function-boundary.md`
- Milestone file: `docs/milestones/M0031-cranelift-backend-smoke.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0043-bootstrap-integer-runtime-semantics.md`
  - `docs/adr/ADR-0045-bootstrap-mir-runtime-contract.md`
  - `docs/adr/ADR-0046-bootstrap-abi-and-calling-convention.md`
  - `docs/adr/ADR-0055-bootstrap-type-environment-transport.md`
- Changed files:
  - `crates/compiler/src/backend.rs`
  - `crates/compiler/tests/backend.rs`
- Ordinary test results:
  - formatter, Clippy, workspace tests, and the focused backend validator passed.

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
Attack: provide unsupported types, MIR forms, or undefined return values.
Expected result: no unsupported state reaches generated Cranelift IR.
Actual result: explicit lowering errors reject every unsupported boundary.
Source of truth: ADR-0043, ADR-0045, ADR-0046, ADR-0055.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/compiler/tests/backend.rs`
- Tests run:
  - `cargo test -p compiler --test backend m0031_lowers_int_constant_return_to_verified_cranelift_ir`
- Result:
  - pass

## Findings

None.

## Ambiguities

None.

## Decision

Pass. This boundary admits only the accepted `Int` literal-return slice and
does not alter source safety semantics.
