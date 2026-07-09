# Soundness Report: M0008-001

## Metadata

- Task ID: `M0008-001`
- Milestone: `M0008`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-09`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0008-001-grammar-authority-ledger.md`
- Milestone file: `docs/milestones/M0008-grammar-authority-and-syntax-ambiguity-ledger.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/`
- Changed files:
  - `docs/syntax/grammar-authority-ledger.md`
  - `docs/ambiguities/M0008-declaration-syntax.md`
  - `docs/ambiguities/M0008-type-generic-syntax.md`
  - `docs/ambiguities/M0008-expression-statement-pattern-syntax.md`
- Ordinary test results:
  - `docs/tests/m0008-grammar-authority-ledger.sh` passed.

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
Attack: Treat Kotlin grammar as implicit parser authority.
Expected result: Rejected.
Actual result: Ledger classifies parser constructs as ambiguous unless accepted ADR authority exists.
Source of truth: docs/syntax/grammar-authority-ledger.md
Outcome: pass

Attack: Add parser fixtures for ambiguous constructs.
Expected result: Rejected.
Actual result: No tests/fixtures/parser directory was added.
Source of truth: docs/milestones/M0008-grammar-authority-and-syntax-ambiguity-ledger.md
Outcome: pass

Attack: Implement parser code during planning milestone.
Expected result: Rejected.
Actual result: No parser.rs or ast.rs exists.
Source of truth: docs/milestones/M0008-grammar-authority-and-syntax-ambiguity-ledger.md
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0008-grammar-authority-ledger.sh`
- Tests run:
  - `docs/tests/m0008-grammar-authority-ledger.sh`
- Result:
  - pass

## Findings

No soundness findings.

The task records syntax ambiguity and does not change compiler behavior or safety semantics.

## Ambiguities

- Declaration syntax remains open.
- Type and generic syntax remains open.
- Expression, statement, and pattern syntax remains open.

## Decision

Pass.
