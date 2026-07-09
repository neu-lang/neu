# Soundness Report: M0007-006

## Metadata

- Task ID: `M0007-006`
- Milestone: `M0007`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-09`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0007-006-accept-lexical-grammar-adr.md`
- Milestone file: `docs/milestones/M0007-lexer-implementation.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0021-lexical-grammar.md`
- Changed files:
  - `docs/adr/ADR-0021-lexical-grammar.md`
  - `docs/SPEC.md`
  - `docs/ambiguities/M0006-lexical-grammar.md`
  - `docs/adr/proposals/reviews/ADR-0021-chief-architect-decision.md`
  - `docs/tests/m0007-lexical-grammar-accepted.sh`
- Ordinary test results:
  - Accepted grammar and resolved-state M0007 validators passed.

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
Attack: Resolve the ambiguity without accepted source of truth.
Expected result: Rejected.
Actual result: docs/adr/ADR-0021-lexical-grammar.md exists with Status: Accepted and docs/SPEC.md summarizes it.
Source of truth: docs/adr/ADR-0021-lexical-grammar.md
Outcome: pass

Attack: Sneak lexer implementation into the semantic decision task.
Expected result: Rejected.
Actual result: No lexer module, token module, or concrete lexer fixtures were added.
Source of truth: docs/tasks/M0007-006-accept-lexical-grammar-adr.md
Outcome: pass

Attack: Leave lexical diagnostics unspecified.
Expected result: Rejected.
Actual result: ADR-0021 defines lexical error categories and source span rules under ADR-0015.
Source of truth: docs/adr/ADR-0021-lexical-grammar.md
Outcome: pass

Attack: Import Kotlin grammar wholesale.
Expected result: Rejected.
Actual result: ADR-0021 defines a small Kotlin-like custom lexical grammar and explicit deferrals.
Source of truth: docs/adr/ADR-0021-lexical-grammar.md
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0007-lexical-grammar-accepted.sh`
- Tests run:
  - `docs/tests/m0007-lexical-grammar-accepted.sh`
  - `docs/tests/m0007-blocker-status-sync.sh`
  - `docs/tests/m0007-language-designer-review.sh`
  - `docs/tests/m0007-lexical-grammar-review.sh`
  - `docs/tests/m0007-lexer-blocked.sh`
- Result:
  - pass

## Findings

No soundness findings.

ADR-0021 is lexical only and does not change ownership, borrowing, lifetime, thread-safety, coroutine, unsafe, or FFI semantics.

## Ambiguities

- Parser grammar remains outside ADR-0021.
- Unicode identifiers remain explicitly deferred.

## Decision

Pass.
