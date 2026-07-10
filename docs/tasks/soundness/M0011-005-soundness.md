# Soundness Report: M0011-005

## Metadata

- Task ID: `M0011-005`
- Milestone: `M0011`
- Filed By: `main-task adversarial check`
- Date: `2026-07-09`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0011-005-accept-declaration-syntax-adr.md`
- Milestone file: `docs/milestones/M0011-declaration-parser.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0021-lexical-grammar.md`
  - `docs/adr/ADR-0022-declaration-syntax.md`
- Changed files:
  - `docs/adr/ADR-0022-declaration-syntax.md`
  - `docs/SPEC.md`
  - `docs/ambiguities/M0008-declaration-syntax.md`
  - `docs/syntax/grammar-authority-ledger.md`
  - `docs/adr/proposals/reviews/ADR-0022-chief-architect-decision.md`
  - `docs/tests/m0011-declaration-syntax-accepted.sh`
- Ordinary test results:
  - `docs/tests/m0011-declaration-syntax-accepted.sh`
  - `docs/tests/m0011-declaration-syntax-concrete-draft.sh`
  - `docs/tests/m0011-declaration-syntax-review.sh`
  - `docs/tests/m0011-declaration-syntax-proposal.sh`
  - `docs/tests/m0011-declaration-parser-blocked.sh`
  - `docs/tests/m0008-grammar-authority-ledger.sh`

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
Attack: Treat deferred type, generic, expression, statement, or pattern syntax as accepted by ADR-0022.
Expected result: Rejected by ledger and accepted ADR deferral list.
Actual result: Rejected; rows remain ambiguous or deferred outside ADR-0022 declaration shell.
Source of truth: docs/adr/ADR-0022-declaration-syntax.md; docs/syntax/grammar-authority-ledger.md
Outcome: pass

Attack: Implement parser behavior during source-of-truth acceptance.
Expected result: No parser implementation or parser fixtures exist.
Actual result: crates/newlang/src/parser.rs and tests/fixtures/parser remain absent.
Source of truth: docs/tasks/M0011-005-accept-declaration-syntax-adr.md
Outcome: pass

Attack: Use draft proposal as authority after acceptance.
Expected result: Current source-of-truth documents cite accepted ADR-0022.
Actual result: SPEC, ambiguity report, ledger, and main task decision cite docs/adr/ADR-0022-declaration-syntax.md.
Source of truth: docs/main task rules; docs/SPEC.md
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0011-declaration-syntax-accepted.sh`
- Tests run:
  - `docs/tests/m0011-declaration-syntax-accepted.sh`
  - `docs/tests/m0011-declaration-parser-blocked.sh`
  - `docs/tests/m0008-grammar-authority-ledger.sh`
- Result:
  - `pass`

## Findings

None.

## Ambiguities

- Type and generic syntax remain unresolved for M0012.
- Expression, statement, pattern, coroutine, and unsafe block syntax remain unresolved for M0013.
- These ambiguities do not block accepting the ADR-0022 declaration shell.

## Decision

Pass.
