# Soundness Report: M0013-008

## Metadata

- Task ID: `M0013-008`
- Milestone: `M0013`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0013-008-expression-statement-pattern-parser-implementation.md`
- Milestone file: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Changed files:
  - `crates/newlang/src/parser.rs`
  - `crates/newlang/tests/parser.rs`
  - `docs/tests/m0013-expression-statement-pattern-parser-implementation.sh`
  - validator updates.
- Ordinary test results:
  - Focused parser tests and M0013 parser implementation validator pass before this report.

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
Attack: Encode type checking, ownership, borrowing, or flow facts in parser output.
Expected result: parser creates syntax-only AST nodes and diagnostics.
Actual result: parser output still uses flat AstArena node kinds and DiagnosticKind values only.
Source of truth: crates/newlang/src/parser.rs
Outcome: pass
```

```text
Attack: Accept deferred syntax through the body parser.
Expected result: loop, unsafe, coroutine-like, when, indexing, and lambda-like forms remain rejected or diagnosed.
Actual result: parser tests cover deferred forms and validator rejects deferred parser APIs.
Source of truth: crates/newlang/tests/parser.rs and docs/tests/m0013-expression-statement-pattern-parser-implementation.sh
Outcome: pass
```

```text
Attack: Treat assignment as an expression.
Expected result: assignment remains a statement form only.
Actual result: assignment is parsed at statement level; assignment-like malformed expression positions diagnose unsupported expression syntax.
Source of truth: docs/adr/ADR-0024-expression-statement-pattern-syntax.md
Outcome: pass
```

```text
Attack: Use pattern parser support to accept match or when syntax early.
Expected result: pattern parser remains reusable internal syntax only.
Actual result: no public standalone pattern parse API or match/when parser was added.
Source of truth: docs/tasks/M0013-008-expression-statement-pattern-parser-implementation.md
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0013-expression-statement-pattern-parser-implementation.sh`
  - focused Rust parser tests in `crates/newlang/tests/parser.rs`
- Tests run:
  - `cargo test --workspace --test parser -- --nocapture`
  - `docs/tests/m0013-expression-statement-pattern-parser-implementation.sh`
  - Full M0013 source-of-truth validators.
- Result:
  - Pass.

## Findings

No soundness findings.

## Ambiguities

- None blocking this parser task.
- Pattern syntax has no accepted external body context until a future match or `when` ADR.

## Decision

Pass.
