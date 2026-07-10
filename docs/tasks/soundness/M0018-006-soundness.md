# Soundness Report: M0018-006

## Metadata

- Task ID: `M0018-006`
- Milestone: `M0018`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0018-006-type-check-output-and-primitives.md`
- Milestone file: `docs/milestones/M0018-type-checking-core.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
  - `docs/adr/ADR-0026-name-resolution-policy.md`
- Changed files:
  - `crates/compiler/src/types.rs`
  - `crates/compiler/src/type_check.rs`
  - `crates/compiler/tests/types.rs`
  - `crates/compiler/tests/type_check.rs`
  - `docs/tasks/M0018-006-type-check-output-and-primitives.md`
- Ordinary test results:
  - `cargo fmt --all --check`: pass
  - `cargo test --workspace --all-targets`: pass, 131 tests
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

Attack: Treat primitive `Int` as an ABI, width, signedness, or backend lowering commitment.
Expected result: No such metadata exists.
Actual result: `PrimitiveType` is only an enum value stored in `TypeKind::Primitive`; no layout, ABI, signedness, width, or backend fields were added.
Source of truth: `docs/adr/ADR-0027-type-checking-core.md` Primitive Type Identity section.
Outcome: pass

Attack: Use side-table lookup to synthesize missing expression or declaration types.
Expected result: Missing node lookups return `None`.
Actual result: Tests assert missing expression, declaration, and assignment entries return `None`.
Source of truth: `docs/adr/ADR-0027-type-checking-core.md` Typed Output Shape section.
Outcome: pass

Attack: Smuggle assignment compatibility into the assignment check table.
Expected result: This task records only accepted assignment checks supplied by callers; it does not decide compatibility.
Actual result: `AssignmentCheck` is an immutable record of statement, target type, and value type; no compatibility function exists.
Source of truth: `docs/tasks/M0018-006-type-check-output-and-primitives.md` Out Of Scope.
Outcome: pass

Attack: Enable direct calls, function type application, ownership, borrow checking, HIR, MIR, or backend behavior.
Expected result: No implementation entry points or data structures for those deferred behaviors are introduced.
Actual result: Changes are limited to primitive type records and type-check report side-table records.
Source of truth: `docs/adr/ADR-0027-type-checking-core.md` Explicit Deferrals.
Outcome: pass

## Adversarial Tests

- Tests added:
  - No separate adversarial Rust tests were needed for this data-model task.
  - Existing unit tests assert no missing-entry synthesis and no primitive metadata beyond identity.
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0018-006-type-check-output-and-primitives.md`
  - `cargo test --workspace --all-targets`
  - `sh docs/tests/m0018-type-checking-core-accepted.sh`
- Result:
  - pass

## Findings

No findings.

## Ambiguities

No new ambiguity found. Literal typing and assignment compatibility remain later M0018 tasks, not blockers for this task.

## Decision

Pass.
