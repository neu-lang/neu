# Soundness Report: M0016-010

## Metadata

- Task ID: `M0016-010`
- Milestone: `M0016`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0016-010-duplicate-declaration-diagnostics.md`
- Milestone file: `docs/milestones/M0016-name-resolution-pass.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- Changed files:
  - `crates/compiler/src/name_resolution.rs`
  - `crates/compiler/tests/name_resolution.rs`
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
Attack: Emit duplicate diagnostics on the original declaration instead of the later attempted declaration.
Expected result: Diagnostic primary span is the later duplicate declaration name span.
Actual result: Tests compare duplicate diagnostic primary span with the second parsed declaration name span.
Source of truth: crates/compiler/tests/name_resolution.rs
Outcome: pass

Attack: Replace the existing declaration when emitting a duplicate diagnostic.
Expected result: Existing declaration remains in the index.
Actual result: Tests confirm the index retains the first declaration after a duplicate.
Source of truth: crates/compiler/tests/name_resolution.rs
Outcome: pass

Attack: Emit duplicate diagnostics for same-name declarations in distinct packages.
Expected result: Different package namespaces are distinct keys and produce no diagnostics.
Actual result: Tests confirm distinct-package same-name declarations both insert and diagnostics are empty.
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

- Related declaration locations remain deferred.

## Decision

Pass for duplicate declaration diagnostics.
