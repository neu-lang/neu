# Soundness Report: M0016-011

## Metadata

- Task ID: `M0016-011`
- Milestone: `M0016`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0016-011-top-level-lookup-query.md`
- Milestone file: `docs/milestones/M0016-name-resolution-pass.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
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
Attack: Return a declaration from a different package.
Expected result: Lookup requires exact package namespace.
Actual result: Tests query the wrong package and receive Unresolved.
Source of truth: crates/compiler/tests/name_resolution.rs
Outcome: pass

Attack: Return a declaration with the wrong declaration kind.
Expected result: Lookup requires exact declaration kind.
Actual result: Tests query the wrong kind and receive Unresolved.
Source of truth: crates/compiler/tests/name_resolution.rs
Outcome: pass

Attack: Hide missing top-level names without a diagnostic.
Expected result: Missing lookup returns unresolved_name with the query span.
Actual result: Tests confirm Unresolved diagnostic kind and primary span.
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

- Local lexical lookup orchestration remains deferred.

## Decision

Pass for exact top-level lookup query API.
