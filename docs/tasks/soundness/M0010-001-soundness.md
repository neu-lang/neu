# Soundness Report: M0010-001

## Metadata

- Task ID: `M0010-001`
- Milestone: `M0010`
- Filed By: `main-task adversarial check`
- Date: `2026-07-09`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0010-001-parser-recovery-architecture.md`
- Milestone file: `docs/milestones/M0010-parser-recovery-architecture.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0021-lexical-grammar.md`
- Changed files:
  - `docs/parser/recovery.md`
  - `docs/parser/syntax-diagnostic-fixtures.md`
  - `tests/fixtures/diagnostics/M0010-synthetic-parser-error.fixture.toml`
  - `tests/golden/diagnostics/M0010-synthetic-parser-error.diagnostic.toml`
- Ordinary test results:
  - `docs/tests/m0010-parser-recovery-architecture.sh` passed.

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
Attack: Encode concrete grammar in parser diagnostic fixtures.
Expected result: Rejected.
Actual result: Fixture uses synthetic token names only and omits language syntax keywords.
Source of truth: docs/syntax/grammar-authority-ledger.md
Outcome: pass

Attack: Add parser implementation during recovery architecture milestone.
Expected result: Rejected.
Actual result: No parser.rs and no parser fixture directory were added.
Source of truth: docs/milestones/M0010-parser-recovery-architecture.md
Outcome: pass

Attack: Promise semantic diagnostics under parser recovery.
Expected result: Rejected.
Actual result: Documentation limits M0010 to syntax diagnostics and synthetic parser error shape.
Source of truth: docs/adr/ADR-0015-diagnostics-as-semantics.md
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0010-parser-recovery-architecture.sh`
- Tests run:
  - `docs/tests/m0010-parser-recovery-architecture.sh`
- Result:
  - pass

## Findings

No soundness findings.

M0010 adds no executable parser behavior and does not affect ownership, borrowing, concurrency, unsafe, or FFI semantics.

## Ambiguities

- Declaration syntax remains open.
- Type and generic syntax remains open.
- Expression, statement, and pattern syntax remains open.

## Decision

Pass.
