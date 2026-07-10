# Soundness Report: M0018-017

## Metadata

- Task ID: `M0018-017`
- Milestone: `M0018`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0018-017-assignment-statement-type-checking.md`
- Milestone file: `docs/milestones/M0018-type-checking-core.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Changed files:
  - `crates/compiler/src/type_check.rs`
  - `crates/compiler/tests/type_check.rs`
- Ordinary test results:
  - `cargo test --workspace --all-targets`: pass, 158 tests

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
Attack: Assignment has missing target or value expression type.
Expected result: No assignment check and no inferred type.
Actual result: Test confirms missing side-table entries are skipped.
Source of truth: ADR-0027 requires no successful type table entry for untyped constructs.
Outcome: pass

Attack: Null value assigned to non-null target.
Expected result: Type mismatch on the value expression.
Actual result: Test confirms mismatch diagnostic with expected and actual type IDs.
Source of truth: ADR-0027 says Null is never assignment-compatible with non-nullable targets.
Outcome: pass

Attack: Nullable exceptions are over-broadened into subtyping.
Expected result: Only Null to nullable target and base to its nullable wrapper are accepted.
Actual result: Compatibility helper only accepts exact identity, nullable target with Null value, or nullable target whose base is the value type.
Source of truth: ADR-0027 assignment compatibility.
Outcome: pass

Attack: Assignment checking enforces mutability, target legality, ownership, borrow, coroutine, HIR, MIR, or backend behavior.
Expected result: No such behavior is introduced.
Actual result: Changed code only consumes parser assignment metadata and expression type side tables.
Source of truth: Task out-of-scope list and ADR-0027 explicit deferrals.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/compiler/tests/type_check.rs`
- Tests run:
  - `cargo test --workspace --all-targets`
  - `docs/scripts/adversarial-check.sh docs/tasks/M0018-017-assignment-statement-type-checking.md`
- Result:
  - Pass.

## Findings

No findings.

## Ambiguities

- None.

## Decision

Pass. The change implements only accepted assignment compatibility over already-known types and does not add safety-phase semantics.
