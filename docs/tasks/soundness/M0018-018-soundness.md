# Soundness Report: M0018-018

## Metadata

- Task ID: `M0018-018`
- Milestone: `M0018`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0018-018-accepted-expression-type-composition.md`
- Milestone file: `docs/milestones/M0018-type-checking-core.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Changed files:
  - `crates/newlang/src/type_check.rs`
  - `crates/newlang/tests/type_check.rs`
- Ordinary test results:
  - `cargo test --workspace --all-targets`: pass, 161 tests

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
Attack: Unknown resolved name appears in the resolution table.
Expected result: No expression type is synthesized.
Actual result: Test confirms the unknown name is skipped.
Source of truth: ADR-0027 requires known type metadata for name expression typing.
Outcome: pass

Attack: Grouped expression inner type is unknown.
Expected result: No grouped expression type is synthesized.
Actual result: Test confirms the grouped expression is skipped.
Source of truth: ADR-0027 says grouped expressions type to the inner expression type.
Outcome: pass

Attack: Composition attempts to type unsupported calls, members, binary expressions, unary expressions, or if expressions.
Expected result: No unsupported expression typing is added.
Actual result: Implementation consumes only literal metadata, grouped metadata, and supplied resolution records.
Source of truth: ADR-0027 explicit deferrals.
Outcome: pass

Attack: Composition runs name resolution, checks assignments, creates declaration signatures, or introduces ownership, borrow, HIR, MIR, or backend behavior.
Expected result: No such behavior is introduced.
Actual result: Changed code only emits expression type side-table entries and a primitive arena.
Source of truth: Task out-of-scope list and ADR-0027 explicit deferrals.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/newlang/tests/type_check.rs`
- Tests run:
  - `cargo test --workspace --all-targets`
  - `docs/scripts/adversarial-check.sh docs/tasks/M0018-018-accepted-expression-type-composition.md`
- Result:
  - Pass.

## Findings

No findings.

## Ambiguities

- None.

## Decision

Pass. The change composes only accepted expression side-table typing and does not invent missing type or safety semantics.
