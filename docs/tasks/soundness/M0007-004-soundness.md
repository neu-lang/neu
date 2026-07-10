# Soundness Report: M0007-004

## Metadata

- Task ID: `M0007-004`
- Milestone: `M0007`
- Filed By: `main-task adversarial check`
- Date: `2026-07-09`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0007-004-lexical-grammar-language-designer-review.md`
- Milestone file: `docs/milestones/M0007-lexer-implementation.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/`
- Changed files:
  - `docs/tests/m0007-language-designer-review.sh`
  - `docs/adr/proposals/reviews/ADR-0021-language-designer-review.md`
- Ordinary test results:
  - `docs/tests/m0007-language-designer-review.sh` passed.

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
Attack: Use main-task semantic design review as accepted lexical grammar.
Expected result: Rejected.
Actual result: Review states it is not accepted source of truth and must not implement lexer behavior.
Source of truth: docs/main task rules, docs/adr/proposals/reviews/ADR-0021-language-designer-review.md
Outcome: pass

Attack: Close M0007 blocker after ownership review alone.
Expected result: Rejected.
Actual result: docs/ambiguities/M0006-lexical-grammar.md remains open and main task decision remains pending.
Source of truth: docs/ambiguities/M0006-lexical-grammar.md
Outcome: pass

Attack: Add concrete lexer fixtures based on review requirements.
Expected result: Rejected.
Actual result: No concrete lexer fixtures or lexer modules were added.
Source of truth: docs/milestones/M0007-lexer-implementation.md
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0007-language-designer-review.sh`
- Tests run:
  - `docs/tests/m0007-language-designer-review.sh`
  - `docs/tests/m0007-lexical-grammar-review.sh`
  - `docs/tests/m0007-lexical-grammar-proposal.sh`
  - `docs/tests/m0007-lexer-blocked.sh`
- Result:
  - pass

## Findings

No soundness findings.

The task is a governance artifact only and does not weaken memory, ownership, borrowing, thread-safety, coroutine, unsafe, or diagnostic invariants.

## Ambiguities

- The lexical grammar ambiguity remains open in `docs/ambiguities/M0006-lexical-grammar.md`.

## Decision

Pass.
