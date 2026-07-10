# Soundness Report: M0009-001

## Metadata

- Task ID: `M0009-001`
- Milestone: `M0009`
- Filed By: `main-task adversarial check`
- Date: `2026-07-09`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0009-001-ast-span-shell.md`
- Milestone file: `docs/milestones/M0009-ast-data-model.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/`
- Changed files:
  - `crates/compiler/src/ast.rs`
  - `crates/compiler/tests/ast.rs`
  - `docs/ast/data-model.md`
- Ordinary test results:
  - `cargo test --workspace --all-targets` passed.

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
Attack: Add concrete syntax AST nodes despite ambiguous grammar.
Expected result: Rejected.
Actual result: AST contains only SourceFile node kind.
Source of truth: docs/syntax/grammar-authority-ledger.md
Outcome: pass

Attack: Encode semantic analysis concepts in AST.
Expected result: Rejected.
Actual result: AST stores node ID, kind, and ByteSpan only.
Source of truth: docs/milestones/M0009-ast-data-model.md
Outcome: pass

Attack: Add parser code during AST milestone.
Expected result: Rejected.
Actual result: No parser.rs or parser fixtures were added.
Source of truth: docs/milestones/M0009-ast-data-model.md
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/compiler/tests/ast.rs`
  - `docs/tests/m0009-ast-data-model.sh`
- Tests run:
  - `cargo test --workspace --all-targets`
  - `docs/tests/m0009-ast-data-model.sh`
- Result:
  - pass

## Findings

No soundness findings.

The AST shell is data-only, uses no unsafe code, and does not affect ownership, borrowing, concurrency, unsafe, or FFI semantics.

## Ambiguities

- Concrete declaration, type, expression, statement, and pattern nodes remain blocked by M0008 ambiguity reports.

## Decision

Pass.
