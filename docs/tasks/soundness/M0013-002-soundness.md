# Soundness Report: M0013-002

## Metadata

- Task ID: `M0013-002`
- Milestone: `M0013`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0013-002-expression-statement-pattern-syntax-proposal.md`
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
  - `docs/adr/proposals/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/tasks/M0013-002-expression-statement-pattern-syntax-proposal.md`
  - `docs/tests/m0013-expression-statement-pattern-syntax-proposal.sh`
- Ordinary test results:
  - M0013 proposal and blocker validators pass before this report.

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
Attack: Treat the draft proposal as accepted parser authority.
Expected result: rejected by non-authority notice and validator.
Actual result: proposal states no parser implementation may depend on it until accepted.
Source of truth: docs/adr/proposals/ADR-0024-expression-statement-pattern-syntax.md
Outcome: pass
```

```text
Attack: Resolve the M0013 ambiguity implicitly by adding proposal text.
Expected result: ambiguity remains open.
Actual result: validator requires ambiguity status to remain open and no accepted ADR-0024 to exist.
Source of truth: docs/tests/m0013-expression-statement-pattern-syntax-proposal.sh
Outcome: pass
```

```text
Attack: Smuggle unsafe or coroutine syntax into implementation through the proposal.
Expected result: proposal can list required decisions but cannot add parser implementation.
Actual result: no parser APIs, AST nodes, or fixtures were added.
Source of truth: docs/tests/m0013-expression-statement-pattern-syntax-proposal.sh
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0013-expression-statement-pattern-syntax-proposal.sh`
- Tests run:
  - M0013 proposal validator
  - M0013 blocker validator
- Result:
  - Pass.

## Findings

No soundness findings.

## Ambiguities

- `docs/ambiguities/M0008-expression-statement-pattern-syntax.md` remains open.
- Concrete grammar, diagnostics, recovery, coroutine syntax, and unsafe block syntax remain unaccepted.

## Decision

Pass.
