# Soundness Report: M0012-004

## Metadata

- Task ID: `M0012-004`
- Milestone: `M0012`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0012-004-type-generic-syntax-concrete-draft.md`
- Milestone file: `docs/milestones/M0012-type-and-generic-syntax-parser.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0016-generics-and-parametric-polymorphism.md`
- Changed files:
  - `docs/adr/proposals/ADR-0023-type-and-generic-syntax.md`
  - `docs/tests/m0012-type-generic-syntax-concrete-draft.sh`
- Ordinary test results:
  - `docs/tests/m0012-type-generic-syntax-concrete-draft.sh`

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
Attack: Treat concrete draft as accepted syntax.
Expected result: Rejected; accepted ADR path remains absent and ambiguity remains open.
Actual result: Validator requires accepted ADR-0023 absence, SPEC absence, and M0012 ambiguity open.
Source of truth: docs/tests/m0012-type-generic-syntax-concrete-draft.sh
Outcome: pass

Attack: Leave nullable function type binding ambiguous.
Expected result: Draft resolves (T) -> U? and ((T) -> U)? cases.
Actual result: Proposal records both attack cases and draft binding.
Source of truth: docs/adr/proposals/ADR-0023-type-and-generic-syntax.md
Outcome: pass

Attack: Use comma-separated bounds ambiguously.
Expected result: Draft chooses & for multiple bounds and rejects comma form.
Actual result: Proposal resolves fun f<T: Send & Share>(); and marks fun f<T: Send, Share>(); malformed.
Source of truth: docs/adr/proposals/ADR-0023-type-and-generic-syntax.md
Outcome: pass

Attack: Smuggle parser implementation.
Expected result: Type parser APIs, type AST nodes, and type/generic fixtures remain absent.
Actual result: Validator confirms absence.
Source of truth: docs/tests/m0012-type-generic-syntax-concrete-draft.sh
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0012-type-generic-syntax-concrete-draft.sh`
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0012-004-type-generic-syntax-concrete-draft.md`
  - `docs/tests/m0012-type-generic-syntax-concrete-draft.sh`
- Result:
  - `pass`

## Findings

None.

## Ambiguities

- ADR-0023 remains draft-only until Chief Architect acceptance.
- M0012 parser implementation remains blocked until accepted source of truth exists.

## Decision

Pass.
