# Soundness Report: M0018-011

## Metadata

- Task ID: `M0018-011`
- Milestone: `M0018`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0018-011-primitive-declaration-initializer-checks.md`
- Milestone file: `docs/milestones/M0018-type-checking-core.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Changed files:
  - `crates/compiler/src/type_check.rs`
  - `crates/compiler/tests/type_check.rs`
  - `docs/tasks/M0018-011-primitive-declaration-initializer-checks.md`
- Ordinary test results:
  - `cargo fmt --all --check`: pass
  - `cargo test --workspace --all-targets`: pass, 142 tests
  - `cargo clippy --workspace --all-targets -- -D warnings`: pass
  - `sh docs/tests/m0018-type-checking-core-accepted.sh`: pass
  - `sh docs/tests/m0002-workspace-ci.sh`: pass

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

Attack: Treat mismatched primitive literal initializers as accepted.
Expected result: A `type_mismatch` diagnostic is recorded and no assignment check is recorded.
Actual result: Tests assert `Bool = 1` records `TypeMismatch` on the initializer node with expected `Bool` and actual `Int`.
Source of truth: `docs/adr/ADR-0027-type-checking-core.md` Type Checking Diagnostics.
Outcome: pass

Attack: Guess types for non-literal initializers or non-primitive declarations.
Expected result: Unknown sides are skipped rather than guessed.
Actual result: Tests assert custom annotations and call initializers receive no assignment checks.
Source of truth: `docs/adr/ADR-0027-type-checking-core.md` Typed Output Shape.
Outcome: pass

Attack: Smuggle nullable compatibility, nominal lookup, generic solving, ownership, borrow checking, HIR, MIR, or backend behavior into initializer checks.
Expected result: No such behavior is added.
Actual result: The checker only compares known primitive `TypeId`s for equality and records diagnostics or assignment checks.
Source of truth: `docs/adr/ADR-0027-type-checking-core.md` Explicit Deferrals.
Outcome: pass

## Adversarial Tests

- Tests added:
  - No separate adversarial Rust tests were needed beyond initializer success and mismatch unit tests.
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0018-011-primitive-declaration-initializer-checks.md`
  - `cargo test --workspace --all-targets`
  - `sh docs/tests/m0018-type-checking-core-accepted.sh`
- Result:
  - pass

## Findings

No findings.

## Ambiguities

No new ambiguity found. Nullable compatibility and name-expression typing remain later M0018 tasks.

## Decision

Pass.
