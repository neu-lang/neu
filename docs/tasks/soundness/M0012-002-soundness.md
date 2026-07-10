# Soundness Report: M0012-002

## Metadata

- Task ID: `M0012-002`
- Milestone: `M0012`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0012-002-type-generic-syntax-proposal.md`
- Milestone file: `docs/milestones/M0012-type-and-generic-syntax-parser.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0016-generics-and-parametric-polymorphism.md`
- Changed files:
  - `docs/adr/proposals/ADR-0023-type-and-generic-syntax.md`
  - `docs/tests/m0012-type-generic-syntax-proposal.sh`
  - `docs/tasks/M0012-002-type-generic-syntax-proposal.md`
- Ordinary test results:
  - `docs/tests/m0012-type-generic-syntax-proposal.sh`
  - `docs/tests/m0012-type-generic-parser-blocked.sh`

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
Attack: Treat ADR-0023 proposal as accepted parser authority.
Expected result: Rejected; accepted ADR path must remain absent and ambiguity remains open.
Actual result: Validator requires accepted docs/adr/ADR-0023-type-and-generic-syntax.md to be absent and M0008 type/generic ambiguity to remain open.
Source of truth: docs/main task rules; docs/ambiguities/M0008-type-generic-syntax.md
Outcome: pass

Attack: Smuggle concrete type parser implementation or fixtures into proposal task.
Expected result: Rejected by validator.
Actual result: No parse_type, type AST nodes, type fixtures, or generic fixtures were added.
Source of truth: docs/tasks/M0012-002-type-generic-syntax-proposal.md
Outcome: pass

Attack: Import Kotlin, Rust, or Go grammar as hidden authority.
Expected result: Proposal states it must not rely on external languages or existing behavior as implicit authority.
Actual result: Proposal includes explicit non-authority wording and required accepted-content checklist.
Source of truth: docs/adr/proposals/ADR-0023-type-and-generic-syntax.md
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0012-type-generic-syntax-proposal.sh`
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0012-002-type-generic-syntax-proposal.md`
  - `docs/tests/m0012-type-generic-syntax-proposal.sh`
- Result:
  - `pass`

## Findings

None.

## Ambiguities

- Type syntax, nullable marker placement, generic parameter syntax, generic argument syntax, capability-bound syntax, and function type syntax remain unresolved until ADR-0023 is reviewed and accepted.

## Decision

Pass.
