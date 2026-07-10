# Soundness Report: M0018-002

## Metadata

- Task ID: `M0018-002`
- Milestone: `M0018`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0018-002-type-checking-core-proposal.md`
- Milestone file: `docs/milestones/M0018-type-checking-core.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/ADR-0026-name-resolution-policy.md`
- Changed files:
  - `docs/adr/proposals/ADR-0027-type-checking-core.md`
  - `docs/tests/m0018-type-checking-core-proposal.sh`
- Ordinary test results:
  - `sh docs/tests/m0018-type-checking-core-proposal.sh`: pass
  - `sh docs/tests/m0018-type-checking-ambiguity-blocker.sh`: pass

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
Attack: Treat the proposal as accepted implementation authority.
Expected result: The proposal has a non-authority notice and accepted ADR-0027 is absent.
Actual result: Validator requires draft status and absence of docs/adr/ADR-0027-type-checking-core.md.
Source of truth: main task rules and M0018 ambiguity report.
Outcome: pass
```

```text
Attack: Sneak compiler behavior into a proposal task.
Expected result: No type checking implementation appears.
Actual result: Validator rejects check_expression, check_declaration, infer_type, literal_type, resolve_call, check_assignment, TypedExpression, TypedProgram, and WellTyped patterns.
Source of truth: task out-of-scope list.
Outcome: pass
```

```text
Attack: Resolve the ambiguity without accepted source of truth.
Expected result: M0018 ambiguity remains open.
Actual result: Validator requires `Status: open`.
Source of truth: docs/ambiguities/M0018-type-checking-core.md.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0018-type-checking-core-proposal.sh`
- Tests run:
  - `sh docs/tests/m0018-type-checking-core-proposal.sh`
- Result:
  - pass

## Findings

No soundness findings.

## Ambiguities

- M0018 remains blocked until ADR-0027 is reviewed and accepted.

## Decision

Pass. The proposal is non-authoritative and does not weaken safety semantics.
