# Soundness Report: M0015-002

## Metadata

- Task ID: `M0015-002`
- Milestone: `M0015`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0015-002-name-table-infrastructure.md`
- Milestone file: `docs/milestones/M0015-symbol-interning-and-name-tables.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
- Changed files:
  - `crates/compiler/src/module.rs`
  - `crates/compiler/src/symbol.rs`
  - `crates/compiler/tests/symbol.rs`
  - `docs/tests/m0015-name-table-infrastructure.sh`
  - `docs/tests/m0015-symbol-interner.sh`
  - `docs/milestones/M0015-symbol-interning-and-name-tables.md`
  - `docs/tasks/M0015-002-name-table-infrastructure.md`
- Ordinary test results:
  - Focused Rust symbol tests plus M0015 name table and symbol interner validators passed.

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
Attack: Collapse same textual name across distinct modules into one table entry.
Expected result: Key includes module identity and symbol identity.
Actual result: Rust tests insert the same symbol in two modules and retrieve distinct entries.
Source of truth: docs/milestones/M0015-symbol-interning-and-name-tables.md
Outcome: pass

Attack: Use duplicate insertion hook to decide duplicate declaration legality.
Expected result: Table reports duplicate data but does not declare language policy.
Actual result: NameTableInsert::Duplicate preserves existing and attempted entries only; validator rejects resolution-policy terms.
Source of truth: docs/milestones/M0015-symbol-interning-and-name-tables.md
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0015-name-table-infrastructure.sh`
  - name table tests in `crates/compiler/tests/symbol.rs`
- Tests run:
  - `cargo test --workspace --all-targets symbol -- --nocapture && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0015-symbol-interner.sh`
- Result:
  - `pass`

## Findings

No findings.

## Ambiguities

- Duplicate-name legality remains deferred; this task only reports duplicate insertion data.

## Decision

Pass.
