# Soundness Report: M0007-008

## Metadata

- Task ID: `M0007-008`
- Milestone: `M0007`
- Filed By: `main-task adversarial check`
- Date: `2026-07-09`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0007-008-lexer-implementation.md`
- Milestone file: `docs/milestones/M0007-lexer-implementation.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0021-lexical-grammar.md`
- Changed files:
  - `crates/compiler/src/lexer.rs`
  - `crates/compiler/tests/lexer.rs`
  - `docs/tests/m0007-lexer-implementation.sh`
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
Attack: Treat integer overflow as a lexer error.
Expected result: Rejected.
Actual result: Oversized decimal integer lexes as IntDecimal without diagnostics.
Source of truth: docs/adr/ADR-0021-lexical-grammar.md
Outcome: pass

Attack: Accept Unicode identifiers despite ADR-0021 deferral.
Expected result: Rejected with lexical diagnostic.
Actual result: Unicode identifier character reports UnsupportedIdentifierCharacter with source span.
Source of truth: docs/adr/ADR-0021-lexical-grammar.md
Outcome: pass

Attack: Mis-tokenize longest operators.
Expected result: Longest accepted operator spelling wins.
Actual result: Tests cover ++, --, ->, =>, .., and ..<.
Source of truth: docs/adr/ADR-0021-lexical-grammar.md
Outcome: pass

Attack: Add parser or AST behavior under lexer milestone.
Expected result: Rejected.
Actual result: No parser, AST, HIR, MIR, backend, or token module split was added.
Source of truth: docs/milestones/M0007-lexer-implementation.md
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/compiler/tests/lexer.rs`
  - `docs/tests/m0007-lexer-implementation.sh`
- Tests run:
  - `cargo test --workspace --all-targets`
  - `docs/tests/m0007-lexer-implementation.sh`
- Result:
  - pass

## Findings

No soundness findings.

The lexer uses no unsafe code and does not affect ownership, borrowing, thread-safety, coroutine, unsafe, or FFI semantics.

## Ambiguities

- Parser grammar remains out of scope.
- Unicode identifiers remain deferred.

## Decision

Pass.
