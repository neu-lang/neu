# Soundness Report: M0011-007

## Metadata

- Task ID: `M0011-007`
- Milestone: `M0011`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-09`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0011-007-declaration-ast-shell.md`
- Milestone file: `docs/milestones/M0011-declaration-parser.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0022-declaration-syntax.md`
- Changed files:
  - `crates/newlang/src/ast.rs`
  - `crates/newlang/tests/ast.rs`
  - `docs/ast/data-model.md`
  - `docs/tests/m0011-declaration-ast-shell.sh`
- Ordinary test results:
  - `cargo test --workspace --all-targets ast`
  - `docs/tests/m0011-declaration-ast-shell.sh`

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
Attack: Use declaration AST shell to encode name resolution, ownership, type checking, or lowering concepts.
Expected result: Validator rejects semantic names and HIR/MIR modules remain absent.
Actual result: AST contains only node kinds, spans, IDs, and arena constructors; HIR/MIR modules remain absent.
Source of truth: docs/ast/data-model.md; docs/tasks/M0011-007-declaration-ast-shell.md
Outcome: pass

Attack: Introduce deferred type, expression, statement, or pattern AST nodes.
Expected result: Validator rejects deferred node names.
Actual result: Only ADR-0022 declaration shell node kinds were added.
Source of truth: docs/adr/ADR-0022-declaration-syntax.md
Outcome: pass

Attack: Implement parser behavior inside AST task.
Expected result: Parser source and executable parser tests remain absent.
Actual result: crates/newlang/src/parser.rs and crates/newlang/tests/parser.rs remain absent.
Source of truth: docs/tasks/M0011-007-declaration-ast-shell.md
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0011-declaration-ast-shell.sh`
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0011-007-declaration-ast-shell.md`
  - `docs/tests/m0011-declaration-ast-shell.sh`
- Result:
  - `pass`

## Findings

None.

## Ambiguities

- AST child relationships, names, modifiers, type placeholders, and body contents remain intentionally deferred.
- Parser implementation remains a separate M0011 task.

## Decision

Pass.
