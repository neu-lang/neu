# Soundness Report: M0016-004

## Metadata

- Task ID: `M0016-004`
- Milestone: `M0016`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0016-004-name-resolution-concrete-draft.md`
- Milestone file: `docs/milestones/M0016-name-resolution-pass.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/proposals/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
  - `docs/adr/ADR-0022-declaration-syntax.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
- Changed files:
  - `docs/adr/proposals/ADR-0026-name-resolution-policy.md`
  - `docs/tasks/M0016-004-name-resolution-concrete-draft.md`
  - `docs/tests/m0016-name-resolution-concrete-draft.sh`
- Ordinary test results:
  - M0016 concrete draft, policy review, proposal, and blocker validators passed.

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
Attack: Treat the revised concrete draft as accepted semantics.
Expected result: ADR-0026 remains a proposal, accepted ADR-0026 is absent, SPEC is unchanged, and M0016 remains blocked.
Actual result: Proposal status remains draft, docs/adr/ADR-0026-name-resolution-policy.md is absent, SPEC has no ADR-0026 section, and the ambiguity remains open.
Source of truth: docs/adr/proposals/ADR-0026-name-resolution-policy.md, docs/ambiguities/M0016-name-resolution-policy.md
Outcome: pass

Attack: Accidentally activate import or cross-module lookup.
Expected result: Draft must explicitly keep imports syntax-only and cross-module lookup unsupported.
Actual result: Draft lookup and unsupported-form sections state imports remain syntax-only and cross-module lookup remains unsupported.
Source of truth: docs/adr/proposals/ADR-0026-name-resolution-policy.md
Outcome: pass

Attack: Permit silent misbinding through unspecified shadowing, duplicates, or ambiguity.
Expected result: Draft must define local-before-declaration, shadowing, duplicate-name, and ambiguity behavior for later acceptance review.
Actual result: Draft scope, shadowing, duplicate, lookup, and diagnostics sections define those behaviors in non-authoritative form.
Source of truth: docs/tests/m0016-name-resolution-concrete-draft.sh
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0016-name-resolution-concrete-draft.sh`
- Tests run:
  - `docs/tests/m0016-name-resolution-concrete-draft.sh && docs/tests/m0016-name-resolution-policy-review.sh && docs/tests/m0016-name-resolution-policy-proposal.sh && docs/tests/m0016-name-resolution-blocked.sh`
- Result:
  - `pass`

## Findings

No findings.

## Ambiguities

- M0016 remains blocked until ADR-0026 is accepted into source of truth.

## Decision

Pass for the concrete draft proposal revision.
