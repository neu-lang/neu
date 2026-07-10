# Soundness Report: M0016-006

## Metadata

- Task ID: `M0016-006`
- Milestone: `M0016`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0016-006-name-resolution-data-model.md`
- Milestone file: `docs/milestones/M0016-name-resolution-pass.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- Changed files:
  - `crates/newlang/src/name_resolution.rs`
  - `crates/newlang/src/lib.rs`
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
  - M0016 authority validators
- Ordinary test results:
  - Rust name-resolution tests and M0016 data-model/authority validators passed.

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
Attack: Smuggle lookup behavior into the data model.
Expected result: The new module stores resolved names and diagnostics only.
Actual result: Validator rejects lookup, scope stack, import resolver, visibility enforcement, and resolver function names.
Source of truth: docs/tests/m0016-name-resolution-data-model.sh
Outcome: pass

Attack: Silently replace an existing resolved reference.
Expected result: Duplicate inserts preserve the existing record and report the attempted duplicate.
Actual result: ResolutionTable returns ResolutionInsert::Duplicate and keeps the original record.
Source of truth: crates/newlang/tests/name_resolution.rs
Outcome: pass

Attack: Omit an accepted ADR-0026 diagnostic variant.
Expected result: Data model represents unresolved, duplicate, ambiguous, unsupported import, unsupported cross-module, and unsupported member diagnostics.
Actual result: ResolutionDiagnosticKind covers all six variants.
Source of truth: crates/newlang/tests/name_resolution.rs
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Tests run:
  - `cargo test -p newlang --test name_resolution && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh`
- Result:
  - `pass`

## Findings

No findings.

## Ambiguities

- Lookup implementation remains intentionally deferred to later M0016 tasks.

## Decision

Pass for name-resolution data model.
