# Soundness Report: M0013-001

## Metadata

- Task ID: `M0013-001`
- Milestone: `M0013`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0013-001-expression-statement-pattern-syntax-blocker.md`
- Milestone file: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0007-error-handling.md`
  - `docs/adr/ADR-0008-structured-concurrency-semantics.md`
  - `docs/adr/ADR-0009-async-suspension-and-borrowing.md`
  - `docs/adr/ADR-0011-flow-typing-and-smart-casts.md`
  - `docs/adr/ADR-0012-pattern-matching-and-algebraic-data.md`
  - `docs/adr/ADR-0018-unsafe-ffi-and-trust-boundaries.md`
  - `docs/adr/ADR-0021-lexical-grammar.md`
- Changed files:
  - `docs/tasks/M0013-001-expression-statement-pattern-syntax-blocker.md`
  - `docs/tests/m0013-expression-statement-pattern-parser-blocked.sh`
  - `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
- Ordinary test results:
  - M0013 blocker validator and M0008 ledger validator pass before this report.

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
Attack: Infer expression precedence from lexer operator tokens.
Expected result: blocked until accepted grammar defines precedence and associativity.
Actual result: ambiguity report remains open and validator requires expression grammar to stay ambiguous.
Source of truth: docs/ambiguities/M0008-expression-statement-pattern-syntax.md
Outcome: pass
```

```text
Attack: Infer pattern syntax from ADR-0012 exhaustive matching semantics.
Expected result: blocked until accepted pattern grammar exists.
Actual result: ledger classifies pattern grammar as ambiguous and validator rejects pattern AST or parser APIs.
Source of truth: docs/syntax/grammar-authority-ledger.md
Outcome: pass
```

```text
Attack: Add coroutine or unsafe parser syntax because semantic ADRs exist.
Expected result: blocked until concrete syntax is accepted.
Actual result: validator rejects coroutine and unsafe parser markers while ledger keeps both ambiguous.
Source of truth: docs/tests/m0013-expression-statement-pattern-parser-blocked.sh
Outcome: pass
```

```text
Attack: Add concrete parser fixtures before grammar authority exists.
Expected result: fixture paths for expressions, statements, and patterns remain absent.
Actual result: validator requires those fixture directories to be absent.
Source of truth: docs/tests/m0013-expression-statement-pattern-parser-blocked.sh
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0013-expression-statement-pattern-parser-blocked.sh`
- Tests run:
  - M0013 blocker validator
  - M0008 grammar authority ledger validator
- Result:
  - Pass.

## Findings

No soundness findings.

## Ambiguities

- `docs/ambiguities/M0008-expression-statement-pattern-syntax.md` remains open and blocks M0013 implementation.

## Decision

Pass.
