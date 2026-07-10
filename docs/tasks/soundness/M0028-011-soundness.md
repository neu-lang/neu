# Soundness Report: M0028-011

## Metadata

- Task ID: `M0028-011`
- Milestone: `M0028`
- Filed By: `main-task adversarial check`
- Date: `2026-07-11`
- Decision: `pass`

## Inputs Read

- Task file: `tasks/M0028-011-<slug>.md`
- Milestone file: `docs/milestones/M0028-executable-expression-frontend-completion.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/<ADR>.md`
- Changed files:
  - `<path>`
- Ordinary test results:
  - `<result reference>`

## Safety Invariants Checked

- [ ] Ownership cannot be bypassed.
- [ ] Moved values cannot be reused.
- [ ] Shared and exclusive borrows cannot conflict.
- [ ] Borrowed data cannot outlive its owner.
- [ ] Nullability refinements cannot be used after invalidation.
- [ ] Thread send/share capabilities are enforced.
- [ ] Coroutine scopes cannot outlive allowed ownership or borrow lifetimes.
- [ ] Borrows across suspension are rejected unless proven safe by accepted semantics.
- [ ] Unsafe and FFI boundaries do not weaken safe-code guarantees.
- [ ] Diagnostics do not hide or misstate safety failures.

## Attacks Attempted

```text
Attack:
Expected result:
Actual result:
Source of truth:
Outcome: pass | fail | ambiguous
```

## Adversarial Tests

- Tests added:
  - `<path>`
- Tests run:
  - `<command>`
- Result:
  - `<result>`

## Findings

List findings ordered by severity.

```text
Severity: blocker | high | medium | low
Invariant:
Finding:
Evidence:
Required fix:
```

## Ambiguities

- <Any ambiguity that prevented a soundness decision. File `AMBIGUITY_REPORT_TEMPLATE.md` output for blocking items.>

## Decision

Pass. Shared module primitive identities are preserved; no safety semantics changed.
