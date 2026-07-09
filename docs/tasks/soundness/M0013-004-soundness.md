# Soundness Report: M0013-004

## Metadata

- Task ID: `M0013-004`
- Milestone: `M0013`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0013-004-expression-statement-pattern-concrete-draft.md`
- Milestone file: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/proposals/ADR-0024-expression-statement-pattern-syntax.md`
- Changed files:
  - `docs/adr/proposals/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/tests/m0013-expression-statement-pattern-syntax-concrete-draft.sh`
- Ordinary test results:
  - M0013 concrete draft, review, proposal, and blocker validators pass before this report.

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
Attack: Treat concrete draft grammar as accepted parser authority.
Expected result: rejected by non-authority status and open ambiguity.
Actual result: validator requires no accepted ADR-0024, no SPEC section, and open ambiguity.
Source of truth: docs/tests/m0013-expression-statement-pattern-syntax-concrete-draft.sh
Outcome: pass
```

```text
Attack: Smuggle coroutine or unsafe syntax into M0013.
Expected result: explicit deferral.
Actual result: concrete draft states unsafe block syntax is deferred and coroutine syntax is deferred.
Source of truth: docs/adr/proposals/ADR-0024-expression-statement-pattern-syntax.md
Outcome: pass
```

```text
Attack: Conflate pattern parsing with binding mode or move semantics.
Expected result: parser records syntax only.
Actual result: draft states pattern binding modes, move, borrow, copy, and smart-cast behavior are semantic checks.
Source of truth: docs/adr/proposals/ADR-0024-expression-statement-pattern-syntax.md
Outcome: pass
```

```text
Attack: Add parser fixtures or AST nodes before acceptance.
Expected result: absent.
Actual result: concrete draft validator rejects expression, statement, and pattern fixture directories plus AST/parser markers.
Source of truth: docs/tests/m0013-expression-statement-pattern-syntax-concrete-draft.sh
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0013-expression-statement-pattern-syntax-concrete-draft.sh`
- Tests run:
  - M0013 concrete draft validator
  - M0013 review validator
  - M0013 proposal validator
  - M0013 blocker validator
- Result:
  - Pass.

## Findings

No soundness findings.

## Ambiguities

- `docs/ambiguities/M0008-expression-statement-pattern-syntax.md` remains open.
- ADR-0024 still requires acceptance before parser work.

## Decision

Pass.
