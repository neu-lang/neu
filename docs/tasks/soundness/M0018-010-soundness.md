# Soundness Report: M0018-010

## Metadata

- Task ID: `M0018-010`
- Milestone: `M0018`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0018-010-primitive-local-declaration-signatures.md`
- Milestone file: `docs/milestones/M0018-type-checking-core.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Changed files:
  - `crates/newlang/src/type_check.rs`
  - `crates/newlang/tests/type_check.rs`
  - `docs/tasks/M0018-010-primitive-local-declaration-signatures.md`
- Ordinary test results:
  - `cargo fmt --all --check`: pass
  - `cargo test --workspace --all-targets`: pass, 140 tests
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

Attack: Treat primitive annotation signatures as initializer validation.
Expected result: Resolver records declaration signatures only.
Actual result: Tests assert expression type and assignment check tables remain empty.
Source of truth: `docs/tasks/M0018-010-primitive-local-declaration-signatures.md` Out Of Scope.
Outcome: pass

Attack: Synthesize signatures for unannotated or non-primitive declarations.
Expected result: No signature is recorded.
Actual result: Tests assert unannotated `inferred` and custom `UserId` declarations have no declaration signature.
Source of truth: `docs/adr/ADR-0027-type-checking-core.md` Concrete Type Checking Model.
Outcome: pass

Attack: Smuggle nominal lookup, nullable resolution, generic solving, ownership, borrow checking, HIR, MIR, or backend behavior into primitive resolution.
Expected result: No such behavior is added.
Actual result: Changes are limited to exact primitive name mapping and declaration signature records.
Source of truth: `docs/adr/ADR-0027-type-checking-core.md` Explicit Deferrals.
Outcome: pass

## Adversarial Tests

- Tests added:
  - No separate adversarial Rust tests were needed beyond primitive signature unit tests.
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0018-010-primitive-local-declaration-signatures.md`
  - `cargo test --workspace --all-targets`
  - `sh docs/tests/m0018-type-checking-core-accepted.sh`
- Result:
  - pass

## Findings

No findings.

## Ambiguities

No new ambiguity found. Nullable annotation resolution, initializer checks, and type mismatch diagnostics remain later M0018 tasks.

## Decision

Pass.
