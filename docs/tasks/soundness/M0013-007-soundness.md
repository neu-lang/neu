# Soundness Report: M0013-007

## Metadata

- Task ID: `M0013-007`
- Milestone: `M0013`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0013-007-expression-statement-pattern-ast-shell.md`
- Milestone file: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Changed files:
  - `crates/newlang/src/ast.rs`
  - `crates/newlang/tests/ast.rs`
  - `docs/tests/m0013-expression-statement-pattern-ast-shell.sh`
  - `docs/ast/data-model.md`
  - validator updates.
- Ordinary test results:
  - Focused Rust AST tests and M0013 AST-shell validators pass before this report.

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
Attack: Encode value typing, flow facts, ownership state, or borrow state in AST shell nodes.
Expected result: AST shell has node kind and span only.
Actual result: AST nodes still contain only id, kind, and span.
Source of truth: crates/newlang/src/ast.rs
Outcome: pass
```

```text
Attack: Use body AST nodes to accept deferred syntax such as match, when, loop, coroutine, unsafe block, indexing, or lambda syntax.
Expected result: AST shell includes only ADR-0024 accepted node kinds.
Actual result: no deferred node kind or parser API was added.
Source of truth: docs/tests/m0013-expression-statement-pattern-ast-shell.sh
Outcome: pass
```

```text
Attack: Introduce parser behavior before parser implementation task.
Expected result: parser source has no body parser APIs.
Actual result: `crates/newlang/src/parser.rs` was not extended with expression, statement, block, pattern, match, coroutine, or unsafe parser APIs.
Source of truth: docs/tests/m0013-expression-statement-pattern-ast-shell.sh
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0013-expression-statement-pattern-ast-shell.sh`
  - focused Rust AST shell test in `crates/newlang/tests/ast.rs`
- Tests run:
  - `cargo test --workspace --all-targets ast -- --nocapture`
  - `docs/tests/m0013-expression-statement-pattern-ast-shell.sh`
  - M0013 source-of-truth validators.
- Result:
  - Pass.

## Findings

No soundness findings.

## Ambiguities

- None blocking this AST shell task.
- Child relationships, typed AST structures, binding modes, flow typing, ownership analysis, borrow analysis, unsafe block syntax, coroutine syntax, loops, and match or when syntax remain deferred.

## Decision

Pass.
