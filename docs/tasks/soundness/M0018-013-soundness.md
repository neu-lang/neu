# Soundness Report: M0018-013

## Metadata

- Task ID: `M0018-013`
- Milestone: `M0018`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0018-013-known-local-symbol-types.md`
- Milestone file: `docs/milestones/M0018-type-checking-core.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0026-name-resolution-policy.md`
- Changed files:
  - `crates/compiler/src/type_check.rs`
  - `crates/compiler/tests/type_check.rs`
- Ordinary test results:
  - `cargo test --workspace --all-targets`: pass, 146 tests

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
Attack: Provide a local binding without a declaration signature.
Expected result: No KnownSymbolType is emitted and no type is inferred.
Actual result: Test confirms the binding is skipped.
Source of truth: ADR-0027 says M0018 does not infer missing declarations.
Outcome: pass

Attack: Provide a declaration signature whose declaration node has no local binding.
Expected result: No KnownSymbolType is synthesized.
Actual result: Test confirms orphan signatures are ignored.
Source of truth: ADR-0027 side-table output and M0016 local binding authority.
Outcome: pass

Attack: Reorder declaration signatures relative to local bindings.
Expected result: Output follows local binding order, not signature order.
Actual result: Test confirms local binding order is preserved.
Source of truth: Task scope and ADR-0027 side-table architecture.
Outcome: pass

Attack: Use known local symbol derivation to trigger expression typing, assignment checks, diagnostics, ownership, borrow, HIR, MIR, or backend behavior.
Expected result: No such behavior is introduced.
Actual result: Changed implementation only returns `Vec<KnownSymbolType>` from supplied side tables.
Source of truth: ADR-0027 explicit deferrals and M0018 milestone scope.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/compiler/tests/type_check.rs`
- Tests run:
  - `cargo test --workspace --all-targets`
  - `docs/scripts/adversarial-check.sh docs/tasks/M0018-013-known-local-symbol-types.md`
- Result:
  - Pass.

## Findings

No findings.

## Ambiguities

- None.

## Decision

Pass. The change does not add safety-phase semantics and does not infer or synthesize missing type information.
