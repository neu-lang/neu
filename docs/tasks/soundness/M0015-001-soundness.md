# Soundness Report: M0015-001

## Metadata

- Task ID: `M0015-001`
- Milestone: `M0015`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0015-001-symbol-interner.md`
- Milestone file: `docs/milestones/M0015-symbol-interning-and-name-tables.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
- Changed files:
  - `crates/compiler/src/lib.rs`
  - `crates/compiler/src/symbol.rs`
  - `crates/compiler/tests/symbol.rs`
  - `docs/tests/m0015-symbol-interner.sh`
  - `docs/milestones/M0015-symbol-interning-and-name-tables.md`
  - `docs/tasks/M0015-001-symbol-interner.md`
- Ordinary test results:
  - Focused Rust symbol tests and M0015 symbol interner validator passed.

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
Attack: Smuggle name resolution or scope policy into the interner.
Expected result: Interner stores exact text and stable IDs only.
Actual result: Validator rejects name-table, scope, import, visibility, type-check, overload, resolution, duplicate, module, and package terms in symbol.rs.
Source of truth: docs/milestones/M0015-symbol-interning-and-name-tables.md
Outcome: pass

Attack: Make symbol identity nondeterministic by using hash iteration order.
Expected result: IDs follow first insertion order.
Actual result: Rust tests prove first inserted text gets ID 0, second gets ID 1, and stored symbols preserve insertion order.
Source of truth: docs/milestones/M0015-symbol-interning-and-name-tables.md
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/compiler/tests/symbol.rs`
  - `docs/tests/m0015-symbol-interner.sh`
- Tests run:
  - `cargo test --workspace --all-targets symbol -- --nocapture && docs/tests/m0015-symbol-interner.sh`
- Result:
  - `pass`

## Findings

No findings.

## Ambiguities

- Duplicate-name behavior remains deferred to a later M0015 task; it does not block symbol interning.

## Decision

Pass.
