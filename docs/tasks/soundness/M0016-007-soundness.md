# Soundness Report: M0016-007

## Metadata

- Task ID: `M0016-007`
- Milestone: `M0016`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0016-007-top-level-declaration-index.md`
- Milestone file: `docs/milestones/M0016-name-resolution-pass.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
- Changed files:
  - `crates/compiler/src/name_resolution.rs`
  - `crates/compiler/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Ordinary test results:
  - Rust name-resolution tests, M0016 data-model validator, and M0016 accepted-state validator passed.

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
Attack: Collapse declarations with the same text across modules or packages.
Expected result: Module and package namespace participate in the declaration key.
Actual result: Tests insert the same symbol across distinct modules and packages without collision.
Source of truth: crates/compiler/tests/name_resolution.rs
Outcome: pass

Attack: Collapse function and type declarations into one key.
Expected result: Declaration kind participates in the key.
Actual result: Tests insert the same symbol as Function and Type under the same module/package without collision.
Source of truth: crates/compiler/tests/name_resolution.rs
Outcome: pass

Attack: Replace an earlier declaration silently on duplicate key insertion.
Expected result: Duplicate insert preserves existing declaration and reports the attempted duplicate.
Actual result: DeclarationIndex returns DeclarationInsert::Duplicate and leaves the original declaration in place.
Source of truth: crates/compiler/tests/name_resolution.rs
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/compiler/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Tests run:
  - `cargo test -p compiler --test name_resolution && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh`
- Result:
  - `pass`

## Findings

No findings.

## Ambiguities

- Parser name extraction and actual declaration collection remain deferred.

## Decision

Pass for top-level declaration index data model.
