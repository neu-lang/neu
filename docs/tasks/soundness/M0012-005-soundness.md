# Soundness Report: M0012-005

## Metadata

- Task ID: `M0012-005`
- Milestone: `M0012`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0012-005-accept-type-generic-syntax-adr.md`
- Milestone file: `docs/milestones/M0012-type-and-generic-syntax-parser.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0016-generics-and-parametric-polymorphism.md`
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
- Changed files:
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
  - `docs/SPEC.md`
  - `docs/ambiguities/M0008-type-generic-syntax.md`
  - `docs/syntax/grammar-authority-ledger.md`
  - `docs/adr/proposals/reviews/ADR-0023-chief-architect-decision.md`
  - M0012 validators and task metadata
- Ordinary test results:
  - Focused M0012 and M0008 validators pass before this report.

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
Attack: Treat accepted capability-bound syntax as accepted capability semantics.
Expected result: rejected by scope; ADR-0023 assigns no semantic meaning to capability names.
Actual result: ADR-0023 defers capability analysis and only defines bound syntax.
Source of truth: docs/adr/ADR-0023-type-and-generic-syntax.md
Outcome: pass
```

```text
Attack: Use ADR-0023 acceptance to smuggle parser implementation or type AST nodes.
Expected result: no parser APIs, no type AST nodes, and no type/generic fixtures in this task.
Actual result: validators require those artifacts to remain absent.
Source of truth: docs/tasks/M0012-005-accept-type-generic-syntax-adr.md
Outcome: pass
```

```text
Attack: Use accepted type syntax as authority for expression, statement, pattern, coroutine, or unsafe syntax.
Expected result: those syntaxes remain ambiguous or blocked.
Actual result: ledger keeps expression, statement, pattern, coroutine, and unsafe constructs outside ADR-0023 authority.
Source of truth: docs/syntax/grammar-authority-ledger.md
Outcome: pass
```

```text
Attack: Hide unsafe nullable or generic cases behind vague diagnostics.
Expected result: type syntax diagnostics cite ADR-0015 and ADR-0023 and require primary span plus recovery action.
Actual result: accepted ADR-0023 lists required diagnostic categories, primary spans, recovery actions, and citation policy.
Source of truth: docs/adr/ADR-0023-type-and-generic-syntax.md
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0012-type-generic-syntax-accepted.sh`
- Tests run:
  - `docs/tests/m0012-type-generic-syntax-accepted.sh`
  - `docs/tests/m0012-type-generic-syntax-concrete-draft.sh`
  - `docs/tests/m0012-type-generic-syntax-review.sh`
  - `docs/tests/m0012-type-generic-syntax-proposal.sh`
  - `docs/tests/m0012-type-generic-parser-blocked.sh`
  - `docs/tests/m0008-grammar-authority-ledger.sh`
- Result:
  - Pass.

## Findings

No soundness findings.

## Ambiguities

- None blocking this task.
- Expression, statement, pattern, coroutine, unsafe, and deferred type forms remain intentionally outside ADR-0023 authority.

## Decision

Pass.
