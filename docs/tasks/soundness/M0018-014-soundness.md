# Soundness Report: M0018-014

## Metadata

- Task ID: `M0018-014`
- Milestone: `M0018`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0018-014-parser-grouped-expression-metadata.md`
- Milestone file: `docs/milestones/M0018-type-checking-core.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Changed files:
  - `crates/newlang/src/parser.rs`
  - `crates/newlang/tests/parser.rs`
- Ordinary test results:
  - `cargo test --workspace --all-targets`: pass, 148 tests

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
Attack: Use grouped expression metadata to type an expression.
Expected result: No type-checking behavior is added in this task.
Actual result: Changed code only records parser metadata.
Source of truth: ADR-0027 side-table model and task out-of-scope list.
Outcome: pass

Attack: Malformed grouped expression attempts to synthesize metadata.
Expected result: No metadata is recorded for the malformed group.
Actual result: Test confirms only the valid group is recorded.
Source of truth: Parser metadata must reflect accepted parsed constructs only.
Outcome: pass

Attack: Nested grouped expressions reorder metadata by AST construction instead of source order.
Expected result: Metadata preserves source order.
Actual result: Tests confirm outer nested grouped expression appears before its inner grouped expression.
Source of truth: Task scope requires source-order preservation.
Outcome: pass

Attack: Metadata addition changes ownership, borrow, coroutine, unsafe, HIR, MIR, or backend behavior.
Expected result: No safety or backend phase behavior changes.
Actual result: Changed files are limited to parser metadata and parser tests.
Source of truth: ADR-0027 explicit deferrals.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/newlang/tests/parser.rs`
- Tests run:
  - `cargo test --workspace --all-targets`
  - `docs/scripts/adversarial-check.sh docs/tasks/M0018-014-parser-grouped-expression-metadata.md`
- Result:
  - Pass.

## Findings

No findings.

## Ambiguities

- None.

## Decision

Pass. The change exposes parser metadata for later M0018 type checking and does not introduce safety-relevant semantics.
