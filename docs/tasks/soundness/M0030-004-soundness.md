# Soundness Report: M0030-004

## Metadata

- Task ID: `M0030-004`
- Milestone: `M0030`
- Filed By: `main-task adversarial check`
- Date: `2026-07-11`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0030-004-type-environment-transport.md`
- Milestone file: `docs/milestones/M0030-mir-design-and-lowering.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0043-bootstrap-integer-runtime-semantics.md`
  - `docs/adr/ADR-0045-bootstrap-mir-runtime-contract.md`
  - `docs/adr/ADR-0052-bootstrap-module-type-identity.md`
  - `docs/adr/ADR-0055-bootstrap-type-environment-transport.md`
- Changed files:
  - `crates/compiler/src/mir.rs`
  - `crates/compiler/tests/mir.rs`
- Ordinary test results:
  - formatter, Clippy, workspace tests, and focused validator passed.

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
Attack: pass an empty or foreign arena for a valid raw TypeId.
Expected result: no raw-ID runtime interpretation occurs.
Actual result: both cases return UnsupportedRuntimeType.
Source of truth: ADR-0052 and ADR-0055.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/compiler/tests/mir.rs`
- Tests run:
  - `cargo test -p compiler --test mir m0030_hir_to_mir_requires_owning_type_arena`
- Result:
  - pass

## Findings

None.

## Ambiguities

None.

## Decision

Pass. The boundary refuses to lower a type outside its owning module type
environment and introduces no ownership, borrowing, concurrency, or unsafe
behavior.
