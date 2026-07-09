# Soundness Report: M0013-005

## Metadata

- Task ID: `M0013-005`
- Milestone: `M0013`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0013-005-accept-expression-statement-pattern-syntax-adr.md`
- Milestone file: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Changed files:
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/SPEC.md`
  - `docs/ambiguities/M0008-expression-statement-pattern-syntax.md`
  - `docs/syntax/grammar-authority-ledger.md`
  - M0013 validators and task metadata.
- Ordinary test results:
  - M0013 accepted-state and historical validators pass before this report.

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
Attack: Treat accepted syntax as accepted type checking or ownership semantics.
Expected result: rejected by scope; ADR-0024 is parser syntax only.
Actual result: ADR-0024 states move validity, binding mode, smart-cast validity, and block value rules are deferred to later semantic phases.
Source of truth: docs/adr/ADR-0024-expression-statement-pattern-syntax.md
Outcome: pass
```

```text
Attack: Smuggle unsafe or coroutine parser syntax into acceptance.
Expected result: explicit deferral.
Actual result: accepted ADR-0024 and ledger mark unsafe block syntax and coroutine syntax as deferred.
Source of truth: docs/syntax/grammar-authority-ledger.md
Outcome: pass
```

```text
Attack: Add body parser implementation while accepting grammar.
Expected result: no parser APIs, AST nodes, or fixtures in this task.
Actual result: accepted-state validator checks parser, AST, and fixture paths remain absent.
Source of truth: docs/tests/m0013-expression-statement-pattern-syntax-accepted.sh
Outcome: pass
```

```text
Attack: Hide parser diagnostics behind vague categories.
Expected result: accepted diagnostic categories include primary span, recovery action, and citation policy.
Actual result: ADR-0024 contains a diagnostic table and requires ADR-0015 plus ADR-0024 citations.
Source of truth: docs/adr/ADR-0024-expression-statement-pattern-syntax.md
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0013-expression-statement-pattern-syntax-accepted.sh`
- Tests run:
  - M0013 accepted-state validator
  - M0013 concrete draft validator
  - M0013 review validator
  - M0013 proposal validator
  - M0013 authority validator
  - M0008 grammar ledger validator
- Result:
  - Pass.

## Findings

No soundness findings.

## Ambiguities

- None blocking expression, statement, and pattern parser syntax authority.
- Unsafe block syntax, coroutine syntax, match or `when`, loops, and other ADR-0024 deferrals remain blocked until future accepted source of truth defines them.

## Decision

Pass.
