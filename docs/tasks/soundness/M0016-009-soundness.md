# Soundness Report: M0016-009

## Metadata

- Task ID: `M0016-009`
- Milestone: `M0016`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0016-009-build-declaration-index.md`
- Milestone file: `docs/milestones/M0016-name-resolution-pass.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
- Changed files:
  - `crates/newlang/src/name_resolution.rs`
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Ordinary test results:
  - Name-resolution tests, M0016 data-model validator, and M0016 accepted-state validator passed.

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
Attack: Collapse same declaration name across packages.
Expected result: Package namespace from module metadata participates in the declaration key.
Actual result: Tests build same-name functions in distinct packages and both insert successfully.
Source of truth: crates/newlang/tests/name_resolution.rs
Outcome: pass

Attack: Lose duplicate evidence needed for duplicate_name diagnostics.
Expected result: Builder preserves all insertion outcomes, including duplicates, without replacing the existing declaration.
Actual result: Tests observe one inserted result and one duplicate result while the index retains the first declaration.
Source of truth: crates/newlang/tests/name_resolution.rs
Outcome: pass

Attack: Smuggle lookup behavior into builder.
Expected result: Builder only populates declaration index.
Actual result: Validator rejects resolver and lookup names in name_resolution.rs.
Source of truth: docs/tests/m0016-name-resolution-data-model.sh
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

- Missing module/package metadata diagnostics remain deferred.

## Decision

Pass for declaration index builder.
