# Soundness Report: M0018-022

## Metadata

- Task ID: `M0018-022`
- Milestone: `M0018`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0018-022-unsupported-expression-diagnostics.md`
- Milestone file: `docs/milestones/M0018-type-checking-core.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Changed files:
  - `crates/newlang/src/type_check.rs`
  - `crates/newlang/tests/type_check.rs`
- Ordinary test results:
  - `cargo test --workspace --all-targets`: pass, 169 tests

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
Attack: Unsupported expressions are accidentally treated as typed.
Expected result: Unsupported nodes receive diagnostics and no expression type entries.
Actual result: Tests assert the report has diagnostics only, with empty expression, declaration, and assignment tables.
Source of truth: ADR-0027 typed output shape and explicit deferrals.
Outcome: pass

Attack: Accepted expression or statement nodes receive unsupported diagnostics.
Expected result: Literals, names, grouped expressions, blocks, declarations, assignments, and returns are ignored by this helper.
Actual result: Test confirms accepted and non-expression nodes produce no diagnostics.
Source of truth: ADR-0027 included expression forms and excluded expression forms.
Outcome: pass

Attack: The helper infers, resolves, lowers, or executes unsupported expressions.
Expected result: It only scans existing AST node kinds and records diagnostics.
Actual result: Implementation consumes only AstArena nodes and does not call parser, name-resolution, inference, HIR, MIR, or backend logic.
Source of truth: Task out-of-scope list and ADR-0027 explicit deferrals.
Outcome: pass

Attack: Unary expressions are mislabeled as binary expressions.
Expected result: Unary nodes have their own stable deferred rule identifier.
Actual result: Test constructs a unary AST node and verifies UnaryExpressionDeferred.
Source of truth: ADR-0027 excludes unary expressions from M0018.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/newlang/tests/type_check.rs`
- Tests run:
  - `cargo test --workspace --all-targets`
  - `docs/scripts/adversarial-check.sh docs/tasks/M0018-022-unsupported-expression-diagnostics.md`
- Result:
  - Pass.

## Findings

No findings.

## Ambiguities

- None.

## Decision

Pass. The change reports unsupported expression diagnostics without adding unsupported expression semantics.
