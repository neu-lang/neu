# Soundness Report: M0018-015

## Metadata

- Task ID: `M0018-015`
- Milestone: `M0018`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0018-015-grouped-expression-typing.md`
- Milestone file: `docs/milestones/M0018-type-checking-core.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Changed files:
  - `crates/compiler/src/type_check.rs`
  - `crates/compiler/tests/type_check.rs`
- Ordinary test results:
  - `cargo test --workspace --all-targets`: pass, 151 tests

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
Attack: Grouped expression has no known inner expression type.
Expected result: No grouped expression type is emitted.
Actual result: Test confirms the grouped expression is skipped.
Source of truth: ADR-0027 requires no successful type table entry for untyped constructs.
Outcome: pass

Attack: Use grouped expression typing to infer a literal or resolve a name.
Expected result: Helper only consumes supplied expression type side-table records.
Actual result: Implementation performs no literal typing or name resolution.
Source of truth: Task out-of-scope list and ADR-0027 side-table model.
Outcome: pass

Attack: Use grouped expression typing to type unsupported calls, members, binary expressions, unary expressions, or if expressions.
Expected result: Unsupported expression rules remain deferred.
Actual result: Implementation does not inspect expression kind and only propagates already-known inner types.
Source of truth: ADR-0027 explicit deferrals.
Outcome: pass

Attack: Type propagation introduces ownership, borrow, coroutine, unsafe, HIR, MIR, or backend behavior.
Expected result: No safety or backend phase behavior changes.
Actual result: Changed files are limited to type-check side-table propagation and tests.
Source of truth: ADR-0027 explicit deferrals.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/compiler/tests/type_check.rs`
- Tests run:
  - `cargo test --workspace --all-targets`
  - `docs/scripts/adversarial-check.sh docs/tasks/M0018-015-grouped-expression-typing.md`
- Result:
  - Pass.

## Findings

No findings.

## Ambiguities

- None.

## Decision

Pass. The change propagates only existing inner expression type information and does not invent missing type or safety semantics.
